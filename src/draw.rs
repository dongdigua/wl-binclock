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
                MyApp::WIDTH as i32,
                MyApp::HEIGHT as i32,
                MyApp::STRIDE as i32,
                wl_shm::Format::Xrgb8888,
            )
            .unwrap();
        let mut image = ImageReader::open("image/test.png")
            .unwrap()
            .decode()
            .unwrap();
        if state.width != image.width() || state.height != image.height() {
            println!("尺寸不一致，需要缩放");
            image = image.resize(
                state.width,
                state.height,
                image::imageops::FilterType::Nearest,
            );
        }
        for (index, (_, _, rbga)) in image.pixels().enumerate() {
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
