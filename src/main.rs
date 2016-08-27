extern crate rayrust;

use std::fs;
use rayrust::color;
use rayrust::image;


fn color(x: f32, y: f32) -> color::Color {
    color::Color::new(x, y, 0.0, 1.0)
}


fn texture(image: &mut image::Image) {
    for row in 0..image.height() {
        for col in 0..image.width() {
            let x = (row as f32) / (image.height() as f32);
            let y = (col as f32) / (image.width() as f32);
            image.set(row, col, &color(x, y));
        }
    }
}

fn main() {
    let mut img = image::Image::new(800, 600);

    texture(&mut img);

    let mut file = fs::File::create("output.ppm").unwrap();
    img.writeppm(&mut file).expect("Failed to write image file");
}
