use crate::generators::Generators;
use image::{ImageBuffer, Rgb, RgbImage};

mod white_noise;

pub async fn generate(w: u32, h: u32, fname: &str, generator: Generators) -> Result<RgbImage, ()> {
    let imgbuf: RgbImage = ImageBuffer::from_fn(
        w,
        h,
        match generator {
            Generators::White => white_noise::generate,
            _ => |_, _| Rgb([0; 3]),
        },
    );

    imgbuf.save(fname).unwrap();

    Ok(RgbImage::new(w, h))
}
