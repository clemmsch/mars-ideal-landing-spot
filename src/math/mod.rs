use std::error::Error;

use crate::colour::RGB;

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub fn hex_to_rgb(input: u32) -> Result<RGB, String> {
    let instr = &*format!("{:0>6x?}", input);
    //println!("{}", instr);
    if instr.len() != 6 {
        return Err("Bad Input Length".to_owned());
    } else {
        return Ok(hex::decode(instr)
            .map(|bytes| RGB::new(input, bytes[0], bytes[1], bytes[2]))
            .unwrap());
    }
}

pub fn euclidian_distance(a: RGB, b: RGB) -> f64 {
    let fst = (a.r as i64 - b.r as i64).pow(2);
    let snd = (a.g as i64 - b.g as i64).pow(2);
    let trd = (a.b as i64 - b.b as i64).pow(2);
    let end = (fst + snd + trd) as f64;
    end.sqrt()
}
