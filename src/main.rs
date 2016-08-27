extern crate rayrust;

use std::fs;
use rayrust::image;


fn main() {
    let img = image::Image::new(800, 600);

    let mut file = fs::File::create("output.ppm").unwrap();
    // let mut file = std::io::stdout();
    img.write(&mut file).expect("Failed to write image file");
}
