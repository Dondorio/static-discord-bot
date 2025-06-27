use image::{ImageBuffer, Rgb, RgbImage};

pub async fn generate_white_noise(w: u32, h: u32) -> Result<RgbImage, ()> {
    // let mut rng = rand::rng();

    let f = |_: u32, _: u32| {
        let val = rand::random::<u8>();
        Rgb([val; 3])
    };

    let imgbuf: RgbImage = ImageBuffer::from_fn(w, h, f);

    imgbuf.save("./img.png").unwrap();

    Ok(RgbImage::new(w, h))
}
