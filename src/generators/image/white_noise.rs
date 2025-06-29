use image::Rgb;

pub fn generate(_: u32, _: u32) -> Rgb<u8> {
    let val = rand::random::<u8>();
    Rgb([val; 3])
}
