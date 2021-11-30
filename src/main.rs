use std::time::Instant;

use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::Rgba;

use crate::colour::Color;
use crate::colour::*;
use crate::matcher::find_images;
use crate::matcher::find_overlaps;
use crate::matcher::parse_image;
use crate::matcher::parse_images;
use crate::matcher::rgb_color_to_normal_color_images;
use crate::math::euclidian_distance;
use crate::math::hex_to_rgb;
use crate::math::rgb_to_hex;

mod colour;
mod matcher;
mod math;

const IMAGE_WIDTH: u32 = 520;
const IMAGE_HEIGHT: u32 = 246;

fn main() {
    let now = Instant::now();
    let images = find_images("img");
    let rgb_colors = parse_images(images);
    let major_colors_of_images: Vec<Vec<Color>> = rgb_color_to_normal_color_images(rgb_colors);
    println!("{}", &major_colors_of_images.len());
    let overlaps = find_overlaps(major_colors_of_images);
    println!("{:#?}", overlaps.len());

    // Save Image
    let mut template = image::open("img/templates/template.png").unwrap();
    for position in overlaps {
        // You would find the position by calculating
        // x + (IMAGE_WIDTH * y) = position
        // Therefore, the following must hold true for x:
        // position % IMAGE_WIDTH = x
        // And the following will hold true for y:
        // (position - x) / 520 = y
        let x = position as u32 % IMAGE_WIDTH;
        let y = (position as u32 - x) / IMAGE_WIDTH;
        println!("x: {}, y: {} :: Position (orig): {}, Position (recalc): {}", x, y, position, (x + (IMAGE_WIDTH * y)));
        template.put_pixel(x, y, (Rgba([255 as u8, 0 as u8, 255 as u8, 255 as u8])));
    }
    template.save("out.png");

    println!("Finished job in {}s", now.elapsed().as_secs_f64());
}
