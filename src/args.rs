use std::num::ParseIntError;

use crate::draw::Palette;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// 0xAARRGGBB or image
    #[clap(long, short, num_args(1..), default_value = "0xff000000", value_parser = parse_palette)]
    pub fg: Vec<Palette>,
    /// 0xAARRGGBB or image
    #[clap(long, short, num_args(1..), default_value = "0xffffffff", value_parser = parse_palette)]
    pub bg: Vec<Palette>,
    /// bitfield top=1 bottom=2 left=4 right=8
    #[clap(long, short, default_value = "9")]
    pub anchor: u32,
}

fn parse_palette(s: &str) -> Result<Palette, ParseIntError> {
    match s.strip_prefix("0x") {
        Some(stripped) =>
            match u32::from_str_radix(stripped, 16) {
                Ok(i) => Ok(Palette::Color(i)),
                Err(e) => Err(e)
            }
        None =>
            Ok(Palette::Image(s.to_string())),
    }
}
