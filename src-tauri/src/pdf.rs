/// PDF baytlarını analiz eder ve SVG formatına çevirir.
/// Eğer PDF vektörel çizim komutları içeriyorsa, bu operatörleri doğrudan SVG yoluna dönüştürür.

pub fn is_pdf(bytes: &[u8]) -> bool {
    bytes.starts_with(b"%PDF")
}

/// Basit ve harici bağımlılık gerektirmeyen PDF vektör operatör ayrıştırıcısı
pub fn parse_pdf_to_svg(pdf_bytes: &[u8]) -> Result<String, String> {
    let content = String::from_utf8_lossy(pdf_bytes);

    // MediaBox veya BBox tespit et
    let mut width = 800.0;
    let mut height = 600.0;

    if let Some(pos) = content.find("/MediaBox") {
        let rest = &content[pos + 9..];
        if let Some(start) = rest.find('[') {
            if let Some(end) = rest[start..].find(']') {
                let box_str = &rest[start + 1..start + end];
                let parts: Vec<f64> = box_str
                    .split_whitespace()
                    .filter_map(|s| s.parse::<f64>().ok())
                    .collect();
                if parts.len() >= 4 {
                    width = (parts[2] - parts[0]).abs();
                    height = (parts[3] - parts[1]).abs();
                }
            }
        }
    }

    // Stream blokları içindeki vektör komutlarını topla (m, l, c, re, h, S)
    let mut svg_paths = Vec::new();
    let mut current_path = String::new();

    // Akış içindeki token'ları tara
    let tokens: Vec<&str> = content.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "m" if i >= 2 => {
                if let (Ok(x), Ok(y)) = (tokens[i - 2].parse::<f64>(), tokens[i - 1].parse::<f64>()) {
                    let svg_y = height - y;
                    current_path.push_str(&format!("M {:.2} {:.2} ", x, svg_y));
                }
            }
            "l" if i >= 2 => {
                if let (Ok(x), Ok(y)) = (tokens[i - 2].parse::<f64>(), tokens[i - 1].parse::<f64>()) {
                    let svg_y = height - y;
                    current_path.push_str(&format!("L {:.2} {:.2} ", x, svg_y));
                }
            }
            "c" if i >= 6 => {
                if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2), Ok(x3), Ok(y3)) = (
                    tokens[i - 6].parse::<f64>(),
                    tokens[i - 5].parse::<f64>(),
                    tokens[i - 4].parse::<f64>(),
                    tokens[i - 3].parse::<f64>(),
                    tokens[i - 2].parse::<f64>(),
                    tokens[i - 1].parse::<f64>(),
                ) {
                    current_path.push_str(&format!(
                        "C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2} ",
                        x1, height - y1, x2, height - y2, x3, height - y3
                    ));
                }
            }
            "re" if i >= 4 => {
                if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                    tokens[i - 4].parse::<f64>(),
                    tokens[i - 3].parse::<f64>(),
                    tokens[i - 2].parse::<f64>(),
                    tokens[i - 1].parse::<f64>(),
                ) {
                    let svg_y = height - (y + h);
                    current_path.push_str(&format!(
                        "M {:.2} {:.2} h {:.2} v {:.2} h {:.2} Z ",
                        x, svg_y, w, h, -w
                    ));
                }
            }
            "h" => {
                current_path.push_str("Z ");
            }
            "S" | "s" | "f" | "F" | "B" | "b" => {
                if !current_path.trim().is_empty() {
                    svg_paths.push(current_path.trim().to_string());
                    current_path.clear();
                }
            }
            _ => {}
        }
        i += 1;
    }

    if !current_path.trim().is_empty() {
        svg_paths.push(current_path.trim().to_string());
    }

    if svg_paths.is_empty() {
        return Err("PDF içerisinde doğrudan vektörel yol bulunamadı".to_string());
    }

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.2} {:.2}\" width=\"{:.2}\" height=\"{:.2}\">\n",
        width, height, width, height
    );

    for p in svg_paths {
        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"0.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\" />\n",
            p
        ));
    }

    svg.push_str("</svg>");
    Ok(svg)
}
