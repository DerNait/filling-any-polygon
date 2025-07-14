// main.rs

mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    let width = 800;
    let height = 600;
    let background_color = Color::WHITE;

    let mut framebuffer = Framebuffer::new(width, height, background_color);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();
    
    let mut output_file = "out.bmp";
    framebuffer.render_to_file(output_file);

    output_file = "out.png";
    framebuffer.render_to_file(output_file);
}
