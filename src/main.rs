use std::time::Instant;

use image::GenericImage;
use image::Rgba;

use crate::colour::Color;
use crate::colour::*;
use crate::matcher::find_images;
use crate::matcher::find_overlaps;
use crate::matcher::parse_images;
use crate::matcher::rgb_color_to_normal_color_images;

mod math;
mod colour;
mod matcher;

const IMAGE_WIDTH: u32 = 520;
const IMAGE_HEIGHT: u32 = 246;

fn main() {
    let now = Instant::now();
    let subdir = std::env::args().nth(1).unwrap(); // Get subdir - e.g. cargo run downscaled -> img/downscaled
    let index = std::env::args().nth(2).unwrap();
    
    let images = find_images(&format!("img/{}/{}", index, subdir));
    let rgb_colors = parse_images(images);
    let major_colors_of_images: Vec<Vec<Color>> = rgb_color_to_normal_color_images(rgb_colors);
    let overlaps = find_overlaps(major_colors_of_images);
    println!("{:#?}", overlaps.len());

    // Save Image
    let template_path = format!("img/{}/templates/template.png", index);

    let mut template = image::open(template_path).unwrap();

    // Draw Equator
    for x in 0..IMAGE_WIDTH {
        template.put_pixel(x, IMAGE_HEIGHT / 2, Rgba([0, 0, 0, 255]));
    }

    // Draw Colors
    for (position, color) in overlaps {
        // You would find the position by calculating
        // x + (IMAGE_WIDTH * y) = position
        // Therefore, the following must hold true for x:
        // position % IMAGE_WIDTH = x
        // And the following will hold true for y:
        // (position - x) / 520 = y
        let x = position as u32 % IMAGE_WIDTH;
        let y = (position as u32 - x) / IMAGE_WIDTH;
        
        let min_pixel_y = (IMAGE_HEIGHT / 2) as f64 - ((IMAGE_HEIGHT / 2) as f64 * 0.5);
        let max_pixel_y = (IMAGE_HEIGHT / 2) as f64 + ((IMAGE_HEIGHT / 2) as f64 * 0.5);
        let opacity = if y > max_pixel_y.ceil() as u32 || y < min_pixel_y.ceil() as u32 { true } else { false };
        
        template.put_pixel(x, y, rgba_from_color(color, opacity));
    }
    template.save(&format!("out.{}.{}.png", index, subdir)).unwrap();

    println!("Finished job in {}s", now.elapsed().as_secs_f64());
}
