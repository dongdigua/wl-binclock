use smithay_client_toolkit::{
    reexports::client::protocol::wl_shm::{self},
    shm::slot::{Buffer, SlotPool},
};
// use chrono::{Local, Timelike};
use rand::prelude::*;

use crate::MyApp;

#[derive(Clone, Debug)]
pub enum Palette {
    Color(u32),
    Image(Vec<u32>)
}

pub struct Painter {
    fg: Vec<Palette>,
    bg: Vec<Palette>
}

impl Painter {
    pub fn new(fg: Vec<Palette>, bg: Vec<Palette>) -> Self {
        Self {
            fg, bg
        }
    }
    pub fn draw(&self, state: &MyApp, digits: [u32; 6]) -> Buffer {
        let mut slot_pool = SlotPool::new(MyApp::STORE_SIZE as usize, &state.shm).unwrap();
        let (buffer, arr) = slot_pool
            .create_buffer(
                state.width as i32,
                state.height as i32,
                (state.width * MyApp::PIXEL_SIZE) as i32,
                wl_shm::Format::Argb8888,
            )
            .unwrap();
        self.draw_digits(arr, digits);
        buffer
    }

    fn draw_digits(&self, v: &mut [u8], digits: [u32; 6]) {

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

fn draw_point(v: &mut [u8], x: usize, y: usize, palette: &Vec<Palette>) {
    let mut rng = rand::rng();
    let chosen = palette.choose(&mut rng).unwrap();

    for ys in 0..=15 {
        for xs in 0..=15 {
            let start = (xs+x*16)*4 + (ys+y*16)*64*6;
            let color_bytes: [u8; 4] = match chosen {
                Palette::Color(color) => *color,
                Palette::Image(image) => image[xs+ys*16],
            }.to_ne_bytes();
            v[start..start+4].copy_from_slice(&color_bytes);
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
