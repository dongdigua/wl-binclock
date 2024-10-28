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
                MyApp::WIDTH,
                MyApp::HEIGHT,
                MyApp::STRIDE,
                wl_shm::Format::Xrgb8888,
            )
            .unwrap();
        let a = ImageReader::open("image/test.png")
            .unwrap()
            .decode()
            .unwrap();
        for (index, (_, _, rbga)) in a.pixels().enumerate() {
            let [r, g, b, _a] = rbga.0;
            let pos = index * 4;
            //注意wayland文档。Xrgb8888是采用小端的。所以低位地址存的是绿色。
            arr[pos..pos + 4].copy_from_slice(&[b, g, r, 0]);
        }
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
