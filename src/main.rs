extern crate rayrust;

use std::fs;
use rayrust::color;
use rayrust::image;
use rayrust::stuff;


fn f(x: f32, y: f32) -> color::Color {
    color::Color::new(x, y, 0.0, 1.0)
}

fn main() {
    let mut img = image::Image::new(800, 600);

    stuff::texture(&mut img, f);

    let mut file = fs::File::create("output.ppm").unwrap();
    img.writeppm(&mut file).expect("Failed to write image file");
}
