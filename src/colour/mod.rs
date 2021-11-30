use lazy_static::lazy_static;

use crate::math::{euclidian_distance, hex_to_rgb};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: u32,
}

impl RGB {
    pub fn new(hex: u32, r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, hex }
    }

    pub fn to_major(&self) -> Color {
        match (*self).hex {
            0x808080 => Color::Grey,
            0xff0000 => Color::Red,
            0x00ff00 => Color::Green,
            0xfff000 => Color::Yellow,
            0xffa500 => Color::Orange,
            0x00ffff => Color::Cyan,
            0x8f00ff => Color::Violet,
            0xfab1be => Color::Pink,
            0xffffff => Color::White,
            0x000000 => Color::Black,
            0x0000FF => Color::Blue,
            _ => panic!("Failed to make RGB a major color"),
        }
    }
}

#[repr(usize)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Grey = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Orange = 5,
    Cyan = 6,
    Violet = 7,
    Pink = 8,
    White = 9,
    Black = 10,
    Blue = 11,
}

lazy_static! {
    pub static ref COLOR_LUT: Vec<RGB> = {
        vec![
            /* Grey     */      RGB::new(0x808080, 128, 128, 128),
            /* Red      */      RGB::new(0xff0000, 255, 000, 000),
            /* Green    */      RGB::new(0x00ff00, 000, 255, 000),
            /* Yellow   */      RGB::new(0xfff000, 255, 255, 000),
            /* Orange   */      RGB::new(0xffa500, 255, 165, 000),
            /* Cyan     */      RGB::new(0x00ffff, 000, 255, 255),
            /* Violet   */      RGB::new(0x8f00ff, 143, 000, 255),
            /* Pink     */      RGB::new(0xfab1be, 255, 177, 190),
            /* White    */      RGB::new(0xffffff, 255, 255, 255),
            /* Black    */      RGB::new(0x000000, 000, 000, 000),
            /* Blue     */      RGB::new(0x0000FF, 000, 000, 255),
        ]
    };
}

pub fn nearest_color(input: u32) -> Color {
    let rgb = hex_to_rgb(input).unwrap();
    let mut cur_nearest: RGB = COLOR_LUT[Color::Grey as usize];
    let mut cur_nearest_dst: f64 = f64::INFINITY;

    for color in COLOR_LUT.iter() {
        let dst = euclidian_distance(rgb, *color);
        if dst < cur_nearest_dst {
            cur_nearest = *color;
            cur_nearest_dst = dst;
        }
    }
    let major = cur_nearest.to_major();
    //println!("{:#?} is the nearest with a distance of {}", major, cur_nearest_dst);
    major
}
