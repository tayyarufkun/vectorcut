use image::GenericImageView;

#[derive(Clone, Debug)]
pub struct BinaryImage {
    pub width: usize,
    pub height: usize,
    /// true = siyah (foreground / kesim çizgisi), false = beyaz (arkaplan)
    pub data: Vec<bool>,
}

impl BinaryImage {
    pub fn new(width: usize, height: usize, initial: bool) -> Self {
        Self {
            width,
            height,
            data: vec![initial; width * height],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = val;
        }
    }
}

/// Verilen görsel baytlarını yükler, şeffaflığı beyaz yapar,
/// gri tonlamaya çevirir ve eşik değerine (threshold) göre saf ikili (binary) harita üretir.
pub fn process_to_binary(image_bytes: &[u8], threshold: u8) -> Result<BinaryImage, String> {
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| format!("Görsel formatı çözülemedi: {}", e))?;

    let (width, height) = img.dimensions();
    let width = width as usize;
    let height = height as usize;

    if width == 0 || height == 0 {
        return Err("Görsel boyutları geçersiz (0x0)".into());
    }

    let rgba = img.to_rgba8();
    let mut binary = BinaryImage::new(width, height, false);

    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x as u32, y as u32);
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let a = pixel[3] as f32 / 255.0;

            // Şeffaflığı beyaz zemin üzerine harmanla (Alpha blending over white)
            let blend_r = r * a + 255.0 * (1.0 - a);
            let blend_g = g * a + 255.0 * (1.0 - a);
            let blend_b = b * a + 255.0 * (1.0 - a);

            // Standart algısal parlaklık (Luminance)
            let gray = (0.299 * blend_r + 0.587 * blend_g + 0.114 * blend_b) as u8;

            // Çizim siyah çizgilerden oluştuğu için eşik değerinden küçük olanlar foreground (true)
            if gray <= threshold {
                binary.set(x, y, true);
            }
        }
    }

    Ok(binary)
}
