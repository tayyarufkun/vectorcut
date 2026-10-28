// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autocrop;
mod image_proc;
mod pdf;
mod tracer;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracer::{CornerType, TraceMode, TraceOptions};

#[derive(Debug, Deserialize)]
pub struct ConvertRequest {
    pub mode: Option<String>,
    pub corners: Option<String>,
    pub threshold: Option<u8>,
    pub autocrop: Option<bool>,
    pub add_frame: Option<bool>,
    pub base64: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConvertResponse {
    pub success: bool,
    pub svg: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize)]
pub struct FilePickResponse {
    pub success: bool,
    pub cancelled: bool,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub is_pdf: bool,
    pub data_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SaveResponse {
    pub success: bool,
    pub cancelled: bool,
    pub saved_path: Option<String>,
}

#[tauri::command]
fn convert_image(request: ConvertRequest) -> Result<ConvertResponse, String> {
    let mode = match request.mode.as_deref() {
        Some("outline") => TraceMode::Outline,
        _ => TraceMode::Centerline,
    };

    let corners = match request.corners.as_deref() {
        Some("smooth") => CornerType::Smooth,
        _ => CornerType::Sharp,
    };

    let threshold = request.threshold.unwrap_or(210);
    let should_autocrop = request.autocrop.unwrap_or(true);
    let add_frame = request.add_frame.unwrap_or(false);

    // Baytları al
    let bytes = if let Some(b64) = request.base64 {
        let clean_b64 = if let Some(idx) = b64.find(',') {
            &b64[idx + 1..]
        } else {
            &b64
        };
        BASE64
            .decode(clean_b64.trim())
            .map_err(|e| format!("Base64 çözülemedi: {}", e))?
    } else if let Some(fp) = request.file_path {
        fs::read(&fp).map_err(|e| format!("Dosya okunamadı: {}", e))?
    } else {
        return Err("Görsel verisi sağlanmadı".into());
    };

    // PDF Kontrolü
    if pdf::is_pdf(&bytes) {
        if let Ok(svg) = pdf::parse_pdf_to_svg(&bytes) {
            return Ok(ConvertResponse {
                success: true,
                svg,
                width: 800,
                height: 600,
            });
        }
    }

    // 1. Görüntüyü ikili haritaya (Binary) dönüştür
    let binary = image_proc::process_to_binary(&bytes, threshold)?;

    // 2. Otomatik Kırpma (Autocrop)
    let (processed_binary, (_x, _y, w, h)) = if should_autocrop {
        autocrop::autocrop(&binary, 2)
    } else {
        let w = binary.width;
        let h = binary.height;
        (binary, (0, 0, w, h))
    };

    // 3. Tek Hat (Centerline) veya Kontur (Outline) Vektörleştirme
    let options = TraceOptions {
        mode,
        corners,
        add_frame,
    };

    let svg = tracer::trace_to_svg(&processed_binary, &options);

    Ok(ConvertResponse {
        success: true,
        svg,
        width: w,
        height: h,
    })
}

#[tauri::command]
fn pick_file() -> Result<FilePickResponse, String> {
    let file = rfd::FileDialog::new()
        .add_filter("Görsel ve PDF Çizimleri", &["png", "jpg", "jpeg", "pdf", "PNG", "JPG", "JPEG", "PDF"])
        .set_title("Lazer Kesim Çizimi Seçin")
        .pick_file();

    if let Some(path) = file {
        let path_str = path.to_string_lossy().to_string();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "cizim".into());

        let bytes = fs::read(&path).map_err(|e| format!("Dosya okunamadı: {}", e))?;
        let is_pdf = path_str.to_lowercase().ends_with(".pdf");

        let mime = if is_pdf {
            "application/pdf"
        } else if path_str.to_lowercase().ends_with(".png") {
            "image/png"
        } else {
            "image/jpeg"
        };

        let b64 = BASE64.encode(&bytes);
        let data_uri = format!("data:{};base64,{}", mime, b64);

        Ok(FilePickResponse {
            success: true,
            cancelled: false,
            file_path: Some(path_str),
            file_name: Some(file_name),
            is_pdf,
            data_uri: Some(data_uri),
        })
    } else {
        Ok(FilePickResponse {
            success: false,
            cancelled: true,
            file_path: None,
            file_name: None,
            is_pdf: false,
            data_uri: None,
        })
    }
}

#[tauri::command]
fn save_svg_dialog(
    svg_content: String,
    default_filename: Option<String>,
) -> Result<SaveResponse, String> {
    let def_name = default_filename.unwrap_or_else(|| "luban_tek_hat.svg".into());
    let def_name = if !def_name.to_lowercase().ends_with(".svg") {
        format!("{}.svg", def_name)
    } else {
        def_name
    };

    let file = rfd::FileDialog::new()
        .add_filter("SVG Kesim Dosyası", &["svg"])
        .set_file_name(&def_name)
        .set_title("Luban SVG Dosyasını Kaydet")
        .save_file();

    if let Some(mut path) = file {
        if path.extension().is_none() {
            path.set_extension("svg");
        }
        fs::write(&path, svg_content.as_bytes())
            .map_err(|e| format!("Dosya kaydedilemedi: {}", e))?;

        Ok(SaveResponse {
            success: true,
            cancelled: false,
            saved_path: Some(path.to_string_lossy().to_string()),
        })
    } else {
        Ok(SaveResponse {
            success: false,
            cancelled: true,
            saved_path: None,
        })
    }
}

#[tauri::command]
fn list_quick_folders() -> Vec<String> {
    let mut folders = Vec::new();
    if let Some(home) = dirs_next_desktop() {
        let lazer_dir = home.join("Lazer");
        if lazer_dir.exists() && lazer_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(lazer_dir) {
                for entry in entries.flatten() {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            if !name.starts_with('.') && name != "app" && !name.ends_with(".app") {
                                folders.push(name);
                            }
                        }
                    }
                }
            }
        }
    }
    folders.sort();
    folders
}

#[tauri::command]
fn load_image_file(path_str: String) -> Result<FilePickResponse, String> {
    let path = Path::new(&path_str);
    if !path.exists() {
        return Err("Dosya bulunamadı".into());
    }
    let bytes = fs::read(path).map_err(|e| format!("Dosya okunamadı: {}", e))?;
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "cizim".into());

    let is_pdf = path_str.to_lowercase().ends_with(".pdf");
    let mime = if is_pdf {
        "application/pdf"
    } else if path_str.to_lowercase().ends_with(".png") {
        "image/png"
    } else {
        "image/jpeg"
    };

    let b64 = BASE64.encode(&bytes);
    let data_uri = format!("data:{};base64,{}", mime, b64);

    Ok(FilePickResponse {
        success: true,
        cancelled: false,
        file_path: Some(path_str),
        file_name: Some(file_name),
        is_pdf,
        data_uri: Some(data_uri),
    })
}

fn dirs_next_desktop() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Desktop"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            convert_image,
            pick_file,
            save_svg_dialog,
            list_quick_folders,
            load_image_file
        ])
        .run(tauri::generate_context!())
        .expect("Tauri uygulaması başlatılırken hata oluştu");
}
