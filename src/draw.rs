use smithay_client_toolkit::{
    reexports::client::protocol::wl_shm::{self},
    shm::slot::{Buffer, SlotPool},
};
use chrono::{Local, Timelike};
use rand::prelude::*;

use crate::MyApp;

pub enum Color {
    Mono(u32),
    Multi(Vec<u32>)
}
pub struct Painter {
    fg: Color,
    bg: Color
}

impl Painter {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            fg : fg,
            bg : bg
        }
    }
    pub fn draw(&self, state: &MyApp) -> Buffer {
        let mut slot_pool = SlotPool::new(MyApp::STORE_SIZE as usize, &state.shm).unwrap();
        let (buffer, arr) = slot_pool
            .create_buffer(
                state.width as i32,
                state.height as i32,
                (state.width * MyApp::PIXEL_SIZE) as i32,
                wl_shm::Format::Xrgb8888,
            )
            .unwrap();
        self.draw_time(arr);
        return buffer;
    }

    fn draw_time(&self, v: &mut [u8]) {
        let now = Local::now();
        let (hours, minutes, seconds) = (
            now.time().hour(),
            now.time().minute(),
            now.time().second(),
        );
        let mut digits: [u32; 6] = [0; 6];
        digits[0] = hours / 10;
        digits[1] = hours % 10;
        digits[2] = minutes / 10;
        digits[3] = minutes % 10;
        digits[4] = seconds / 10;
        digits[5] = seconds % 10;

        for (idx, dg) in digits.iter().enumerate() {
            for (idy, b) in mkmask(*dg).iter().enumerate() {
                if *b == 1 {
                    draw_point(v, idx, idy, &self.fg);
                } else if *b == 0 {
                    draw_point(v, idx, idy, &self.bg);
                }
            }
        }
    }

}

fn draw_point(v: &mut [u8], x: usize, y: usize, color: &Color) {
    let mut rng = rand::rng();
    let color_u32 = match color {
        Color::Mono(c) => c,
        Color::Multi(cv) => cv.choose(&mut rng).unwrap()
    };
    let color_bytes: [u8; 4] = color_u32.to_ne_bytes();
    // 6x4 grid (each 16x16 pixel)
    for xs in (x*16)..=(x*16+15) {
        for ys in y*16..=(y*16+15) {
            let start = xs*4 + ys*64*6;
            v[start] = color_bytes[0];
            v[start+1] = color_bytes[1];
            v[start+2] = color_bytes[2];
            v[start+3] = color_bytes[3];
        }
    }
}

// 查表法哈哈
fn mkmask(d: u32) -> [u32; 4]{
    match d {
        0 => [0,0,0,0],
        1 => [0,0,0,1],
        2 => [0,0,1,0],
        3 => [0,0,1,1],
        4 => [0,1,0,0],
        5 => [0,1,0,1],
        6 => [0,1,1,0],
        7 => [0,1,1,1],
        8 => [1,0,0,0],
        9 => [1,0,0,1],
        _ => [1,1,1,1]
    }
}
