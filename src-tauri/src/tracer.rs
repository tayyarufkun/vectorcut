use crate::image_proc::BinaryImage;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TraceMode {
    Centerline,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CornerType {
    Sharp,
    Smooth,
}

#[derive(Clone, Debug)]
pub struct TraceOptions {
    pub mode: TraceMode,
    pub corners: CornerType,
    pub add_frame: bool,
}

/// Zhang-Suen inceltme algoritması ile ikili görseldeki siyah çizgileri
/// 1-piksel kalınlığındaki tek hat iskeletine (centerline) dönüştürür.
pub fn zhang_suen_thinning(binary: &BinaryImage) -> BinaryImage {
    let mut grid = binary.clone();
    let w = grid.width;
    let h = grid.height;

    let mut to_clear = Vec::new();

    loop {
        let mut changed = false;

        // 1. Alt Adım (Sub-iteration 1)
        for y in 0..h {
            for x in 0..w {
                if !grid.get(x, y) {
                    continue;
                }

                let p2 = if y > 0 { grid.get(x, y - 1) as u8 } else { 0 };
                let p3 = if y > 0 && x + 1 < w { grid.get(x + 1, y - 1) as u8 } else { 0 };
                let p4 = if x + 1 < w { grid.get(x + 1, y) as u8 } else { 0 };
                let p5 = if y + 1 < h && x + 1 < w { grid.get(x + 1, y + 1) as u8 } else { 0 };
                let p6 = if y + 1 < h { grid.get(x, y + 1) as u8 } else { 0 };
                let p7 = if y + 1 < h && x > 0 { grid.get(x - 1, y + 1) as u8 } else { 0 };
                let p8 = if x > 0 { grid.get(x - 1, y) as u8 } else { 0 };
                let p9 = if y > 0 && x > 0 { grid.get(x - 1, y - 1) as u8 } else { 0 };

                let b = p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9;
                if !(2..=6).contains(&b) {
                    continue;
                }

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9, p2];
                let mut a = 0;
                for i in 0..8 {
                    if neighbors[i] == 0 && neighbors[i + 1] == 1 {
                        a += 1;
                    }
                }
                if a != 1 {
                    continue;
                }

                if (p2 * p4 * p6 == 0) && (p4 * p6 * p8 == 0) {
                    to_clear.push((x, y));
                }
            }
        }

        if !to_clear.is_empty() {
            changed = true;
            for (x, y) in to_clear.drain(..) {
                grid.set(x, y, false);
            }
        }

        // 2. Alt Adım (Sub-iteration 2)
        for y in 0..h {
            for x in 0..w {
                if !grid.get(x, y) {
                    continue;
                }

                let p2 = if y > 0 { grid.get(x, y - 1) as u8 } else { 0 };
                let p3 = if y > 0 && x + 1 < w { grid.get(x + 1, y - 1) as u8 } else { 0 };
                let p4 = if x + 1 < w { grid.get(x + 1, y) as u8 } else { 0 };
                let p5 = if y + 1 < h && x + 1 < w { grid.get(x + 1, y + 1) as u8 } else { 0 };
                let p6 = if y + 1 < h { grid.get(x, y + 1) as u8 } else { 0 };
                let p7 = if y + 1 < h && x > 0 { grid.get(x - 1, y + 1) as u8 } else { 0 };
                let p8 = if x > 0 { grid.get(x - 1, y) as u8 } else { 0 };
                let p9 = if y > 0 && x > 0 { grid.get(x - 1, y - 1) as u8 } else { 0 };

                let b = p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9;
                if !(2..=6).contains(&b) {
                    continue;
                }

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9, p2];
                let mut a = 0;
                for i in 0..8 {
                    if neighbors[i] == 0 && neighbors[i + 1] == 1 {
                        a += 1;
                    }
                }
                if a != 1 {
                    continue;
                }

                if (p2 * p4 * p8 == 0) && (p2 * p6 * p8 == 0) {
                    to_clear.push((x, y));
                }
            }
        }

        if !to_clear.is_empty() {
            changed = true;
            for (x, y) in to_clear.drain(..) {
                grid.set(x, y, false);
            }
        }

        if !changed {
            break;
        }
    }

    grid
}

type Point = (f64, f64);

/// İskelet piksellerinden komşu noktaları bularak polylinelara dönüştürür
pub fn trace_skeleton_polylines(skeleton: &BinaryImage) -> Vec<Vec<Point>> {
    let w = skeleton.width;
    let h = skeleton.height;
    let mut visited = vec![false; w * h];
    let mut polylines: Vec<Vec<Point>> = Vec::new();

    let dx = [0, 1, 1, 1, 0, -1, -1, -1];
    let dy = [-1, -1, 0, 1, 1, 1, 0, -1];

    let get_neighbors = |x: usize, y: usize, sk: &BinaryImage| -> Vec<(usize, usize)> {
        let mut list = Vec::with_capacity(8);
        for i in 0..8 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];
            if nx >= 0 && nx < sk.width as isize && ny >= 0 && ny < sk.height as isize {
                if sk.get(nx as usize, ny as usize) {
                    list.push((nx as usize, ny as usize));
                }
            }
        }
        list
    };

    // 1. Aşama: Uç noktalar (Endpoints) veya kavşaklar
    for y in 0..h {
        for x in 0..w {
            if skeleton.get(x, y) && !visited[y * w + x] {
                let nbrs = get_neighbors(x, y, skeleton);
                if nbrs.len() == 1 || nbrs.len() > 2 {
                    visited[y * w + x] = true;

                    for &next in &nbrs {
                        if !visited[next.1 * w + next.0] {
                            let mut current = next;
                            let mut branch = vec![(x as f64 + 0.5, y as f64 + 0.5)];

                            while !visited[current.1 * w + current.0] {
                                visited[current.1 * w + current.0] = true;
                                branch.push((current.0 as f64 + 0.5, current.1 as f64 + 0.5));

                                let next_nbrs = get_neighbors(current.0, current.1, skeleton);
                                let unvis: Vec<(usize, usize)> = next_nbrs
                                    .into_iter()
                                    .filter(|&(nx, ny)| !visited[ny * w + nx])
                                    .collect();

                                if unvis.len() == 1 {
                                    current = unvis[0];
                                } else {
                                    break;
                                }
                            }
                            if branch.len() > 1 {
                                polylines.push(branch);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Aşama: Kalan kapalı döngüler (Closed loops)
    for y in 0..h {
        for x in 0..w {
            if skeleton.get(x, y) && !visited[y * w + x] {
                let mut current = (x, y);
                let mut loop_path = Vec::new();

                while !visited[current.1 * w + current.0] {
                    visited[current.1 * w + current.0] = true;
                    loop_path.push((current.0 as f64 + 0.5, current.1 as f64 + 0.5));

                    let nbrs = get_neighbors(current.0, current.1, skeleton);
                    let unvis: Vec<(usize, usize)> = nbrs
                        .into_iter()
                        .filter(|&(nx, ny)| !visited[ny * w + nx])
                        .collect();

                    if let Some(&first) = unvis.first() {
                        current = first;
                    } else {
                        break;
                    }
                }

                if loop_path.len() > 1 {
                    loop_path.push(loop_path[0]);
                    polylines.push(loop_path);
                }
            }
        }
    }

    polylines
}

/// Outline modu için sınır izleme
pub fn trace_outline_polylines(binary: &BinaryImage) -> Vec<Vec<Point>> {
    let w = binary.width;
    let h = binary.height;
    let mut polylines = Vec::new();

    let mut visited_h = vec![false; (w + 1) * (h + 1)];
    let mut visited_v = vec![false; (w + 1) * (h + 1)];

    let is_black = |x: isize, y: isize| -> bool {
        if x >= 0 && x < w as isize && y >= 0 && y < h as isize {
            binary.get(x as usize, y as usize)
        } else {
            false
        }
    };

    for y in 0..=h {
        for x in 0..=w {
            let top_black = is_black(x as isize, y as isize - 1);
            let bot_black = is_black(x as isize, y as isize);

            if top_black != bot_black && !visited_h[y * (w + 1) + x] {
                let mut path = Vec::new();
                let mut cx = x;
                let mut cy = y;

                loop {
                    path.push((cx as f64, cy as f64));
                    let mut found_next = false;

                    if cx < w {
                        let t = is_black(cx as isize, cy as isize - 1);
                        let b = is_black(cx as isize, cy as isize);
                        if t != b && !visited_h[cy * (w + 1) + cx] {
                            visited_h[cy * (w + 1) + cx] = true;
                            cx += 1;
                            found_next = true;
                        }
                    }

                    if !found_next && cy < h {
                        let l = is_black(cx as isize - 1, cy as isize);
                        let r = is_black(cx as isize, cy as isize);
                        if l != r && !visited_v[cy * (w + 1) + cx] {
                            visited_v[cy * (w + 1) + cx] = true;
                            cy += 1;
                            found_next = true;
                        }
                    }

                    if !found_next && cx > 0 {
                        let t = is_black(cx as isize - 1, cy as isize - 1);
                        let b = is_black(cx as isize - 1, cy as isize);
                        if t != b && !visited_h[cy * (w + 1) + (cx - 1)] {
                            visited_h[cy * (w + 1) + (cx - 1)] = true;
                            cx -= 1;
                            found_next = true;
                        }
                    }

                    if !found_next && cy > 0 {
                        let l = is_black(cx as isize - 1, cy as isize - 1);
                        let r = is_black(cx as isize, cy as isize - 1);
                        if l != r && !visited_v[(cy - 1) * (w + 1) + cx] {
                            visited_v[(cy - 1) * (w + 1) + cx] = true;
                            cy -= 1;
                            found_next = true;
                        }
                    }

                    if !found_next || (cx == x && cy == y) {
                        break;
                    }
                }

                if path.len() > 3 {
                    path.push((x as f64, y as f64));
                    polylines.push(path);
                }
            }
        }
    }

    polylines
}

fn point_to_segment_distance(p: Point, a: Point, b: Point) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let len_sq = dx * dx + dy * dy;

    if len_sq == 0.0 {
        let px = p.0 - a.0;
        let py = p.1 - a.1;
        return (px * px + py * py).sqrt();
    }

    let t = ((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj_x = a.0 + t * dx;
    let proj_y = a.1 + t * dy;

    let rx = p.0 - proj_x;
    let ry = p.1 - proj_y;
    (rx * rx + ry * ry).sqrt()
}

pub fn ramer_douglas_peucker(points: &[Point], epsilon: f64) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut max_dist = 0.0;
    let mut index = 0;
    let start = points[0];
    let end = points[points.len() - 1];

    for i in 1..points.len() - 1 {
        let dist = point_to_segment_distance(points[i], start, end);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }

    if max_dist > epsilon {
        let left = ramer_douglas_peucker(&points[..=index], epsilon);
        let right = ramer_douglas_peucker(&points[index..], epsilon);

        let mut result = left;
        result.pop();
        result.extend(right);
        result
    } else {
        vec![start, end]
    }
}

fn points_to_smooth_svg_path(points: &[Point]) -> String {
    if points.len() < 2 {
        return String::new();
    }
    if points.len() == 2 {
        return format!("M {:.2} {:.2} L {:.2} {:.2}", points[0].0, points[0].1, points[1].0, points[1].1);
    }

    let mut d = format!("M {:.2} {:.2}", points[0].0, points[0].1);

    for i in 0..points.len() - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() { points[i + 2] } else { p2 };

        let cp1x = p1.0 + (p2.0 - p0.0) / 6.0;
        let cp1y = p1.1 + (p2.1 - p0.1) / 6.0;
        let cp2x = p2.0 - (p3.0 - p1.0) / 6.0;
        let cp2y = p2.1 - (p3.1 - p1.1) / 6.0;

        d.push_str(&format!(
            " C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
            cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
        ));
    }

    d
}

fn points_to_sharp_svg_path(points: &[Point]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut d = format!("M {:.2} {:.2}", points[0].0, points[0].1);
    for pt in &points[1..] {
        d.push_str(&format!(" L {:.2} {:.2}", pt.0, pt.1));
    }
    d
}

pub fn trace_to_svg(binary: &BinaryImage, options: &TraceOptions) -> String {
    let raw_polylines = match options.mode {
        TraceMode::Centerline => {
            let skeleton = zhang_suen_thinning(binary);
            trace_skeleton_polylines(&skeleton)
        }
        TraceMode::Outline => trace_outline_polylines(binary),
    };

    let epsilon = match options.corners {
        CornerType::Sharp => 1.0,
        CornerType::Smooth => 1.8,
    };

    let simplified: Vec<Vec<Point>> = raw_polylines
        .iter()
        .map(|pts| ramer_douglas_peucker(pts, epsilon))
        .filter(|pts| pts.len() >= 2)
        .collect();

    let w = binary.width;
    let h = binary.height;

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
        w, h, w, h
    );

    for pts in &simplified {
        let d = match options.corners {
            CornerType::Sharp => points_to_sharp_svg_path(pts),
            CornerType::Smooth => points_to_smooth_svg_path(pts),
        };
        if !d.is_empty() {
            svg.push_str(&format!(
                "  <path d=\"{}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"0.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\" />\n",
                d
            ));
        }
    }

    if options.add_frame && w > 1 && h > 1 {
        svg.push_str(&format!(
            "  <rect x=\"0.5\" y=\"0.5\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"0.5\" />\n",
            w - 1,
            h - 1
        ));
    }

    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zhang_suen_thinning_reduces_thick_line() {
        let mut img = BinaryImage::new(30, 30, false);
        // 5 piksel kalınlığında dikey çizgi (x: 10..15, y: 5..25)
        for y in 5..25 {
            for x in 10..15 {
                img.set(x, y, true);
            }
        }

        let skeleton = zhang_suen_thinning(&img);

        let initial_pixels: usize = img.data.iter().map(|&b| b as usize).sum();
        let skeleton_pixels: usize = skeleton.data.iter().map(|&b| b as usize).sum();

        assert_eq!(initial_pixels, 20 * 5);
        // İskelet pikselleri tek hat olduğu için büyük ölçüde azalmalıdır
        assert!(skeleton_pixels < 25, "İskelet piksel sayısı tek hatta inmelidir, bulunan: {}", skeleton_pixels);

        // Orta satırlarda genişlik tam olarak 1 piksel olmalıdır
        for y in 8..22 {
            let row_black: usize = (0..30).map(|x| skeleton.get(x, y) as usize).sum();
            assert_eq!(row_black, 1, "Satır {} tek hat (1 piksel) olmalı", y);
        }
    }

    #[test]
    fn test_trace_to_svg_produces_valid_luban_svg() {
        let mut img = BinaryImage::new(30, 30, false);
        for x in 5..25 {
            img.set(x, 15, true);
        }

        let opts = TraceOptions {
            mode: TraceMode::Centerline,
            corners: CornerType::Sharp,
            add_frame: true,
        };

        let svg = trace_to_svg(&img, &opts);
        assert!(svg.contains("<svg"), "SVG başlangıç etiketi içermeli");
        assert!(svg.contains("stroke=\"#000000\""), "Luban kesim rengi stroke=#000000 olmalı");
        assert!(svg.contains("fill=\"none\""), "Kesim çizgisi fill=none olmalı");
        assert!(svg.contains("<rect"), "Dış kesim çerçevesi eklenmeli");
    }
}
