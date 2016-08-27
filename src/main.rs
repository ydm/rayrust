extern crate rayrust;

use std::fs;
use rayrust::image;


fn color(x: f32, y: f32) -> image::Color {
    
}


fn main() {
    let img = image::Image::new(800, 600);

    let mut file = fs::File::create("output.ppm").unwrap();
    // let mut file = std::io::stdout();
    img.write(&mut file).expect("Failed to write image file");
}
