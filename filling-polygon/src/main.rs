// main.rs

mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;

mod polygon;
use polygon::{draw_polygon_outline, fill_polygon_scanline};

fn main() {
    let width = 800;
    let height = 600;
    let background_color = Color::WHITE;

    let mut framebuffer = Framebuffer::new(width, height, background_color);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let polygon1 = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    framebuffer.set_current_color(Color::YELLOW);
    fill_polygon_scanline(&mut framebuffer, &polygon1);

    framebuffer.set_current_color(Color::WHITE);
    draw_polygon_outline(&mut framebuffer, &polygon1);

    let polygon2 = vec![
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    framebuffer.set_current_color(Color::BLUE);
    fill_polygon_scanline(&mut framebuffer, &polygon2);
    
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon_outline(&mut framebuffer, &polygon2);
    
    let mut output_file = "out.bmp";
    framebuffer.render_to_file(output_file);

    output_file = "out.png";
    framebuffer.render_to_file(output_file);
}
