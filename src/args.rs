use std::num::ParseIntError;

use crate::draw::Palette;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// 0xAARRGGBB or 16x16 image
    #[clap(long, short, num_args(1..), default_value = "0xff000000", value_parser = parse_palette)]
    pub fg: Vec<Palette>,
    /// 0xAARRGGBB or 16x16 image
    #[clap(long, short, num_args(1..), default_value = "0xffffffff", value_parser = parse_palette)]
    pub bg: Vec<Palette>,
    /// bitfield top=1 bottom=2 left=4 right=8
    #[clap(long, short, default_value = "9")]
    pub anchor: u32,
    /// use stdin as timer source
    #[clap(long, short)]
    pub pipe: bool,
}

fn parse_palette(s: &str) -> Result<Palette, ParseIntError> {
    match s.strip_prefix("0x") {
        Some(stripped) =>
            match u32::from_str_radix(stripped, 16) {
                Ok(i) => Ok(Palette::Color(i)),
                Err(e) => Err(e)
            }
        None => {
            // DeepSeek
            let img = image::open(s).expect(&format!("Failed to open image: {}", s));
            let rgba_img = img.into_rgba8();
            let raw_data = rgba_img.into_raw();
            let argb_pixels: Vec<_> = raw_data
                .chunks_exact(4)
                .map(|chunk| {
                    let r = chunk[0];
                    let g = chunk[1];
                    let b = chunk[2];
                    let a = chunk[3];
                    // Pack into u32 as 0xaarrggbb
                    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)
                })
                .collect();
            if argb_pixels.len() != 256 {
                panic!("Please use a 16x16 image!");
            }
            Ok(Palette::Image(argb_pixels))
        }
    }
}
