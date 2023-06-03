#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! image = "0.24.0"
//! ```

use image::GenericImageView;
use image::ImageFormat;

fn main() {
    let img = image::open("assets/sun.png").expect("Failed to open image");

    let (img_width, img_height) = img.dimensions();
    let aspect_ratio = img_width as u32 / img_height as u32;

    let vertex_width: u32 = 5;
    let vertex_height: u32 = 50;

    let (target_width, target_height) = if aspect_ratio > 1 {
        (
            vertex_width,
            (vertex_width as u32 / aspect_ratio) as u32,
        )
    } else {
        (
            (vertex_height as u32 * aspect_ratio) as u32,
            vertex_height,
        )
    };

    let resized_img = img.resize_exact(target_width, target_height, image::imageops::FilterType::Lanczos3);

    resized_img.save_with_format("assets/sun.png", ImageFormat::Png).expect("Failed to save image");
}