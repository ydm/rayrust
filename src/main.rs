extern crate nalgebra as na;
extern crate rayrust;

use std::fs;
use rayrust::camera;
use rayrust::color;
use rayrust::image;
use rayrust::stuff;
use rayrust::primitive;
use rayrust::ray::{ self, Intersectable };


fn main() {
    let width = 800;
    let height = 600;

    let mut img = image::Image::new(width, height);

    // fn f(x: f32, y: f32) -> color::Color {
    //     color::Color::new(x, y, 0.0, 1.0)
    // }
    // stuff::texture(&mut img, f);

    let cam = camera::OrthographicCamera::new((width, height));
    let sphere1 = primitive::Sphere::new(&na::Point4::new(0.0, 0.0,  0.0, 1.0), 0.5);
    let sphere2 = primitive::Sphere::new(&na::Point4::new(0.5, 0.0, -5.0, 1.0), 0.5);

    let background = color::Color::new(1.0, 1.0, 1.0, 1.0);
    let color1 = color::Color::new(1.0, 0.0, 0.0, 1.0);
    let color2 = color::Color::new(0.0, 1.0, 0.0, 1.0);

    for row in 0..img.height() {
        for col in 0..img.width() {
            let ray = cam.generate_ray(row, col);

            img.set(row, col, &background);
            if let Some(x) = sphere1.intersection(&ray) {
                img.set(row, col, &color1);
            } else if let Some(x) = sphere2.intersection(&ray) {
                img.set(row, col, &color2);
            }
        }
    }

    let mut file = fs::File::create("output.ppm").unwrap();
    img.writeppm(&mut file).expect("Failed to write image file");
}
