use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

/// Sólo el contorno
pub fn draw_polygon_outline(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let h = framebuffer.height as f32;

    // Convertir a coordenadas de pantalla (origen arriba-izquierda)
    let pts: Vec<Vector2> = points
        .iter()
        .map(|p| Vector2::new(p.x, h - p.y))
        .collect();

    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        line(framebuffer, a, b);
    }
}

/// Relleno con scanline + bounding-box
pub fn fill_polygon_scanline(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 3 {
        return;
    }

    let h = framebuffer.height as f32;

    // 0) Transformar Y
    let pts: Vec<Vector2> = points
        .iter()
        .map(|p| Vector2::new(p.x, h - p.y))
        .collect();

    // 1) Límite vertical del polígono
    let (mut min_y, mut max_y) = (pts[0].y as i32, pts[0].y as i32);
    for p in &pts {
        let y = p.y as i32;
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    // 2) Recorrer cada scanline
    for y in min_y..=max_y {
        let mut xs: Vec<f32> = Vec::new(); // ← ahora f32

        // 2-a) Cruces con cada arista
        for i in 0..pts.len() {
            let mut p1 = pts[i];
            let mut p2 = pts[(i + 1) % pts.len()];

            // Ignorar aristas horizontales
            if (p1.y - p2.y).abs() < f32::EPSILON {
                continue;
            }

            // Garantizar p1.y < p2.y
            if p1.y > p2.y {
                std::mem::swap(&mut p1, &mut p2);
            }

            // Regla odd/even sin duplicar vértices
            if (y as f32) >= p1.y && (y as f32) < p2.y {
                let t = (y as f32 - p1.y) / (p2.y - p1.y);
                let x = p1.x + t * (p2.x - p1.x);
                xs.push(x);
            }
        }

        // 2-b) Ordenar y rellenar de 2 en 2
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut it = xs.chunks_exact(2);
        while let Some(&[x0, x1]) = it.next() {
            let x_start = x0.ceil() as i32;  // dentro del polígono
            let x_end   = x1.floor() as i32; // dentro del polígono
            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }
        }
    }
}
