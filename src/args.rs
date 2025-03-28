use std::num::ParseIntError;

use crate::draw::Color;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// AARRGGBB
    #[clap(long, short, num_args(1..), default_value = "0xff000000", value_parser = parse_hex)]
    pub fg: Vec<u32>,
    /// AARRGGBB
    #[clap(long, short, num_args(1..), default_value = "0xffffffff", value_parser = parse_hex)]
    pub bg: Vec<u32>,
    /// bitfield top=1 bottom=2 left=4 right=8
    #[clap(long, short, default_value = "9")]
    pub anchor: u32,
}

fn parse_hex(s: &str) -> Result<u32, String> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    u32::from_str_radix(stripped, 16)
        .map_err(|e: ParseIntError| format!("Invalid hexadecimal value '{}': {}", s, e))
}

impl From<Vec<u32>> for Color {
    fn from(values: Vec<u32>) -> Self {
        if values.len() == 1 {
            Color::Mono(values[0])
        } else {
            Color::Multi(values)
        }
    }
}
