use std::collections::HashMap;
use std::hash::Hash;

use image::DynamicImage;
use image::GenericImageView;

use crate::colour::RGB;
use crate::colour::*;
use crate::math::*;

pub fn find_images(dir: &str) -> Vec<DynamicImage> {
    let mut finished: Vec<DynamicImage> = vec![];
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // We do not care about Directories (raw, gimp. Only top-level is considered). We also discard non-PNG files
        if path.is_dir() || path.extension().unwrap() != "png" {
            continue;
        }
        finished.push(image::open(path).unwrap());
    }

    finished
}

pub fn parse_images(images: Vec<DynamicImage>) -> Vec<Vec<RGB>> {
    let mut finished: Vec<Vec<RGB>> = vec![];
    for img in images {
        finished.push(parse_image(img));
    }

    finished
}

pub fn rgb_color_to_normal_color_images(rgb_vecs: Vec<Vec<RGB>>) -> Vec<Vec<Color>> {
    let mut finished = vec![];
    for vec in rgb_vecs {
        let mut majors: Vec<Color> = vec![];
        for x in vec.iter() {
            let major = nearest_color(x.hex);
            // We cannot discard White & Black as images may have
            // a different amount of W&B
            majors.push(major);
        }
        finished.push(majors);
    }

    finished
}

/// # Parse Image
/// Inputs:
///     - path: The Dynamic Image
/// Outputs:
///     Vec<RGB>: The colorvalues of all pixels
pub fn parse_image(img: DynamicImage) -> Vec<RGB> {
    let mut rgb_vec: Vec<RGB> = vec![];
    let rgba = img.to_rgba8();
    //println!("{},{}", img.width(), img.height());
    //println!("{}", rgba.len());
    for pix in rgba.pixels() {
        let tup = pix.0;
        let (r, g, b) = (tup[0], tup[1], tup[2]);
        rgb_vec.push(RGB::new(rgb_to_hex(r, g, b), r, g, b));
    }

    rgb_vec
}

/// # Find Overlaps
/// Inputs:
///     - inputs: Takes in the colors of each pixel of the image
///
/// Outputs:
///     - Vec<u128>: The vector consisting of all overlap points
pub fn find_overlaps(inputs: Vec<Vec<Color>>) -> HashMap<usize, Color> {
    let mut chint = 0; // Checks that it is always checking all images
    // .len() does not work?????
    let shouldbe = inputs.len();
    println!("{}", shouldbe);

    let len_first = inputs[0].len();
    for (idx, vector) in inputs.iter().enumerate() {
        chint += 1;
        if vector.len() != len_first {
            panic!(
                "The vector at inputs[{}] (Len: {}) does not match the size of inputs[0] ({})",
                idx,
                vector.len(),
                len_first
            );
        }
    }
    if chint != shouldbe { panic!("Check-Int should be {}, but is {}", shouldbe, chint); };
    chint = 0;

    let mut overlaps: HashMap<usize, Color> = HashMap::new();
    // We cannot zip() as size of inputs is unknown as of compile time
    for (idx, color) in inputs[0].iter().enumerate() {
        // White and Black may be skipped
        if color == &Color::White || color == &Color::Black {
            continue;
        }

        // We cannot iterate *directly* over inputs again as Vec<> does not implement
        // Copy. Therefore, we have to implement it using this
        let mut is_overlap = true;
        for pos in 0..inputs.len() {
            if !is_overlap { break; }


            chint += 1;
            if &inputs[pos][idx] == color {
                continue;
            } else {
                is_overlap = false;
            }
        }

        if is_overlap {
            overlaps.insert(idx, *color);
        }
    }

//    if chint != shouldbe { panic!("Check-Int should be {}, but is {}", shouldbe, chint); };

    overlaps
}
