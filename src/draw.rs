use smithay_client_toolkit::{
    reexports::client::protocol::wl_shm::{self},
    shm::slot::{Buffer, SlotPool},
};
use chrono::{Local, Timelike};

use crate::MyApp;

pub struct Painter {}

impl Painter {
    pub fn draw(state: &MyApp) -> Buffer {
        let mut slot_pool = SlotPool::new(MyApp::STORE_SIZE as usize, &state.shm).unwrap();
        let (buffer, arr) = slot_pool
            .create_buffer(
                state.width as i32,
                state.height as i32,
                (state.width * MyApp::PIXEL_SIZE) as i32,
                wl_shm::Format::Xrgb8888,
            )
            .unwrap();
        arr.fill(255);
        draw_time(arr);
        dbg!(&buffer);
        return buffer;
    }
}

fn draw_point(v: &mut [u8], x: usize, y: usize) {
    // the cavas 128 * 128 * 4 (pixel size)
    // is divided into 8 * 8 grid (each 16 * 16)
    // 64 = 16 (point_width) * 4 (pixel size)
    for xs in (x*64)..=(x*64+63) {
        for ys in y*16..=(y*16+15) {
            v[xs + ys*512] = 0;
        }
    }
}

fn draw_time(v: &mut [u8]) {
    let now = Local::now();
    let (hours, minutes, seconds) = (
        now.time().hour(),
        now.time().minute(),
        now.time().second(),
    );
    let mut digits: [u32; 8] = [0; 8];
    digits[0] = hours / 10;
    digits[1] = hours % 10;
    digits[2] = minutes / 10;
    digits[3] = minutes % 10;
    digits[4] = seconds / 10;
    digits[5] = seconds % 10;

    for (idx, dg) in digits.iter().enumerate() {
        for (idy, b) in mkmask(*dg).iter().enumerate() {
            if *b == 1 {
                draw_point(v, idx+1, idy+2);
            }
        }
    }
}

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
