use crate::image_proc::BinaryImage;

/// Çizimin siyah piksellerini tarayarak en dış koordinatları (min_x, min_y, max_x, max_y) bulur
/// ve gereksiz dış boşlukları kırpar (Autocrop / Sıkı Çerçeve).
pub fn autocrop(
    binary: &BinaryImage,
    padding: usize,
) -> (BinaryImage, (usize, usize, usize, usize)) {
    let mut min_x = binary.width;
    let mut min_y = binary.height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..binary.height {
        for x in 0..binary.width {
            if binary.get(x, y) {
                found = true;
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }

    if !found {
        return (binary.clone(), (0, 0, binary.width, binary.height));
    }

    let x0 = min_x.saturating_sub(padding);
    let y0 = min_y.saturating_sub(padding);
    let x1 = (max_x + padding + 1).min(binary.width);
    let y1 = (max_y + padding + 1).min(binary.height);

    let new_w = x1 - x0;
    let new_h = y1 - y0;

    let mut cropped = BinaryImage::new(new_w, new_h, false);

    for y in 0..new_h {
        for x in 0..new_w {
            cropped.set(x, y, binary.get(x0 + x, y0 + y));
        }
    }

    (cropped, (x0, y0, new_w, new_h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocrop_tightly_crops_bounds() {
        let mut img = BinaryImage::new(100, 100, false);
        // Sadece (20..30, 40..60) bölgesine piksel koy
        for y in 40..60 {
            for x in 20..30 {
                img.set(x, y, true);
            }
        }

        let pad = 2;
        let (cropped, (x0, y0, w, h)) = autocrop(&img, pad);

        assert_eq!(x0, 20 - pad);
        assert_eq!(y0, 40 - pad);
        assert_eq!(w, 10 + 2 * pad);
        assert_eq!(h, 20 + 2 * pad);
        assert_eq!(cropped.width, w);
        assert_eq!(cropped.height, h);
    }
}

