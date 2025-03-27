use image::{GenericImageView, ImageReader};
use smithay_client_toolkit::{
    reexports::client::protocol::wl_shm::{self},
    shm::slot::{Buffer, SlotPool},
};

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
        arr.fill(128);
        return buffer;
    }
}

#[cfg(test)]
mod tests {
    use image::{GenericImageView, ImageReader};

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let a = ImageReader::open("image/test.png")?.decode()?;
        let en = a.pixels().enumerate();
        for (index, (x, y, p)) in en {
            println!("index:{},x{},y{},p{:?}", index, x, y, p);
        }
        Ok(())
    }
}
