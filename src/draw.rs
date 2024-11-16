use image::{GenericImageView, ImageReader};
use smithay_client_toolkit::{
    reexports::client::protocol::wl_shm::{self},
    shm::slot::{Buffer, SlotPool},
};

use crate::MyApp;

pub struct Painter {}

impl Painter {
    pub fn draw(state: &MyApp) -> Buffer {
        let mut slot_pool = SlotPool::new(state.store_size(), &state.shm).unwrap();
        let (buffer, arr) = slot_pool
            .create_buffer(
                state.width as i32,
                state.height as i32,
                state.stride(),
                wl_shm::Format::Xrgb8888,
            )
            .unwrap();

        let image = resize(state);
        for (index, (_, _, rbga)) in image.pixels().enumerate() {
            let [r, g, b, _a] = rbga.0;
            let pos = index * 4;
            //注意wayland文档。Xrgb8888是采用小端的。所以低位地址存的是绿色。
            arr[pos..pos + 4].copy_from_slice(&[b, g, r, 0]);
        }
        return buffer;
    }
}

fn resize(state: &MyApp) -> image::DynamicImage {
    let path = state.config.get_current_image();
    let image = ImageReader::open(path.clone()).unwrap().decode().unwrap();
    if state.width != image.width() || state.height != image.height() {
        let mut resizer = fast_image_resize::Resizer::new();
        let options = fast_image_resize::ResizeOptions {
            algorithm: fast_image_resize::ResizeAlg::Convolution(
                fast_image_resize::FilterType::Lanczos3,
            ),
            ..Default::default()
        };
        let mut new_image = image::DynamicImage::new(state.width, state.height, image.color());
        resizer.resize(&image, &mut new_image, &options).unwrap();
        new_image.save(path).unwrap();
        new_image
    } else {
        image
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
