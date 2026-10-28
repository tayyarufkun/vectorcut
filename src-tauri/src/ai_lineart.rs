use image::{DynamicImage, GenericImageView, GrayImage, Luma};
use serde::Serialize;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tract_onnx::prelude::*;

const MODEL_FILENAME: &str = "lineart_sketch.onnx";
// HuggingFace üzerinde barındırılan optimize hafif Sketch/LineArt ONNX modeli
const MODEL_DOWNLOAD_URL: &str =
    "https://huggingface.co/lllyasviel/ControlNet/resolve/main/annotator/ckpts/sketch_lines.onnx";

#[derive(Serialize, Clone, Debug)]
pub struct ModelStatus {
    pub installed: bool,
    pub path: String,
    pub size_mb: f64,
}

/// Modelin saklandığı yerel dizini döner (~/.vectorcut/models/lineart_sketch.onnx)
pub fn get_models_dir() -> PathBuf {
    if let Some(mut dir) = dirs_next::data_dir() {
        dir.push("VectorCut");
        dir.push("models");
        dir
    } else if let Some(mut home) = dirs_next::home_dir() {
        home.push(".vectorcut");
        home.push("models");
        home
    } else {
        PathBuf::from("./models")
    }
}

pub fn get_model_path() -> PathBuf {
    get_models_dir().join(MODEL_FILENAME)
}

pub fn check_model_status() -> ModelStatus {
    let p = get_model_path();
    if p.exists() && p.is_file() {
        if let Ok(meta) = fs::metadata(&p) {
            let bytes = meta.len();
            if bytes > 100_000 {
                return ModelStatus {
                    installed: true,
                    path: p.to_string_lossy().to_string(),
                    size_mb: (bytes as f64) / (1024.0 * 1024.0),
                };
            }
        }
    }
    ModelStatus {
        installed: false,
        path: p.to_string_lossy().to_string(),
        size_mb: 0.0,
    }
}

/// Modeli uzaktan indirir
pub fn download_lineart_model<F>(progress_callback: F) -> Result<(), String>
where
    F: Fn(f32) + Send + 'static,
{
    let dir = get_models_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Dizin oluşturulamadı: {}", e))?;
    }
    let target_path = get_model_path();
    let temp_path = dir.join(format!("{}.tmp", MODEL_FILENAME));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("İstemci hatası: {}", e))?;

    let mut response = client
        .get(MODEL_DOWNLOAD_URL)
        .send()
        .map_err(|e| format!("Model indirilemedi: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Sunucu hatası: HTTP {}",
            response.status().as_u16()
        ));
    }

    let total_size = response.content_length().unwrap_or(25_000_000) as f32;
    let mut downloaded = 0.0;
    let mut file = File::create(&temp_path).map_err(|e| format!("Geçici dosya oluşturulamadı: {}", e))?;

    let mut buffer = [0u8; 16384];
    loop {
        let bytes_read = response
            .read(&mut buffer)
            .map_err(|e| format!("Okuma hatası: {}", e))?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])
            .map_err(|e| format!("Yazma hatası: {}", e))?;
        downloaded += bytes_read as f32;
        let progress = (downloaded / total_size).min(1.0);
        progress_callback(progress);
    }

    // Dosyayı asıl hedefe taşı
    if target_path.exists() {
        let _ = fs::remove_file(&target_path);
    }
    fs::rename(&temp_path, &target_path)
        .map_err(|e| format!("Model dosyası taşınamadı: {}", e))?;

    Ok(())
}

pub fn remove_model() -> Result<(), String> {
    let p = get_model_path();
    if p.exists() {
        fs::remove_file(p).map_err(|e| format!("Dosya silinemedi: {}", e))?;
    }
    Ok(())
}

/// Sobel tabanlı akıllı kenar tespiti (Zero-AI fallback - her ortamda anında çalışır)
pub fn smart_edge_detection(image_bytes: &[u8]) -> Result<DynamicImage, String> {
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| format!("Görsel çözülemedi: {}", e))?;

    let (w, h) = img.dimensions();
    let gray = img.to_luma8();
    let mut edges = GrayImage::new(w, h);

    // Beyaz arka plan üzerine siyah çizgiler (LineArt stili)
    for y in 1..h.saturating_sub(1) {
        for x in 1..w.saturating_sub(1) {
            // Sobel Gradient X & Y
            let p00 = gray.get_pixel(x - 1, y - 1)[0] as f32;
            let p01 = gray.get_pixel(x - 1, y)[0] as f32;
            let p02 = gray.get_pixel(x - 1, y + 1)[0] as f32;
            let p20 = gray.get_pixel(x + 1, y - 1)[0] as f32;
            let p21 = gray.get_pixel(x + 1, y)[0] as f32;
            let p22 = gray.get_pixel(x + 1, y + 1)[0] as f32;

            let p10 = gray.get_pixel(x, y - 1)[0] as f32;
            let p12 = gray.get_pixel(x, y + 1)[0] as f32;

            let gx = (p20 + 2.0 * p21 + p22) - (p00 + 2.0 * p01 + p02);
            let gy = (p02 + 2.0 * p12 + p22) - (p00 + 2.0 * p10 + p20);

            let magnitude = (gx * gx + gy * gy).sqrt();
            let val = if magnitude > 45.0 { 0u8 } else { 255u8 };
            edges.put_pixel(x, y, Luma([val]));
        }
    }

    Ok(DynamicImage::ImageLuma8(edges))
}

/// Tract-ONNX motoru ile LineArt AI modelini çalıştırır
pub fn run_ai_lineart(image_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let model_path = get_model_path();
    if !model_path.exists() {
        // Model yüklü değilse akıllı Sobel kenar çıkarımı yap
        let edge_img = smart_edge_detection(image_bytes)?;
        let mut buffer = std::io::Cursor::new(Vec::new());
        edge_img
            .write_to(&mut buffer, image::ImageFormat::Png)
            .map_err(|e| format!("PNG kodlama hatası: {}", e))?;
        return Ok(buffer.into_inner());
    }

    let img = image::load_from_memory(image_bytes)
        .map_err(|e| format!("Görsel okunamadı: {}", e))?;

    let (orig_w, orig_h) = img.dimensions();
    let model_size = 512;
    let resized = img.resize_exact(model_size, model_size, image::imageops::FilterType::Triangle);
    let rgb = resized.to_rgb8();

    // RGB tensörü oluştur [1, 3, 512, 512]
    let tensor: Tensor = tract_ndarray::Array4::from_shape_fn(
        (1, 3, model_size as usize, model_size as usize),
        |(_, c, y, x)| {
            let pixel = rgb.get_pixel(x as u32, y as u32);
            pixel[c] as f32 / 255.0
        },
    )
    .into();

    let model = tract_onnx::onnx()
        .model_for_path(&model_path)
        .map_err(|e| format!("Model yüklenemedi: {}", e))?
        .into_optimized()
        .map_err(|e| format!("Model optimize edilemedi: {}", e))?
        .into_runnable()
        .map_err(|e| format!("Model çalıştırılabilir yapılamadı: {}", e))?;

    let result = model
        .run(tvec!(tensor.into()))
        .map_err(|e| format!("Model çıkarımı başarısız: {}", e))?;

    let output = result[0]
        .to_plain_array_view::<f32>()
        .map_err(|e| format!("Çıktı tensörü okunamadı: {}", e))?;

    let mut out_gray = GrayImage::new(model_size, model_size);
    for y in 0..model_size {
        for x in 0..model_size {
            let val = output[[0, 0, y as usize, x as usize]];
            let pixel_val = ((1.0 - val.clamp(0.0, 1.0)) * 255.0) as u8;
            out_gray.put_pixel(x, y, Luma([pixel_val]));
        }
    }

    let dynamic_gray = DynamicImage::ImageLuma8(out_gray);
    let final_img = dynamic_gray.resize_exact(orig_w, orig_h, image::imageops::FilterType::Triangle);

    let mut buffer = std::io::Cursor::new(Vec::new());
    final_img
        .write_to(&mut buffer, image::ImageFormat::Png)
        .map_err(|e| format!("Görsel kaydedilemedi: {}", e))?;

    Ok(buffer.into_inner())
}
