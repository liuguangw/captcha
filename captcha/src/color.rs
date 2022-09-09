use image::Rgba;
use rand::{rngs::ThreadRng, Rng};
pub fn gen_background_color(rng: &mut ThreadRng) -> Rgba<u8> {
    let red = rng.gen_range(200..=255);
    let green = rng.gen_range(200..=255);
    let blue = rng.gen_range(200..=255);
    //let a=rng.gen_range(0..255);
    Rgba([red, green, blue, 255])
}
pub fn gen_text_color(rng: &mut ThreadRng) -> Rgba<u8> {
    let red = rng.gen_range(0..=150);
    let green = rng.gen_range(0..=150);
    let blue = rng.gen_range(0..=150);
    Rgba([red, green, blue, 255])
}

pub fn gen_line_color(rng: &mut ThreadRng) -> Rgba<u8> {
    let red = rng.gen_range(100..=255);
    let green = rng.gen_range(100..=255);
    let blue = rng.gen_range(100..=255);
    Rgba([red, green, blue, 255])
}
