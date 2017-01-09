extern crate nalgebra as na;
extern crate rayrust;

use std::fs;
use na::{ Point3, Vector3 };
use rayrust::camera::persp;
use rayrust::camera::common::{ Camera };
use rayrust::color;
use rayrust::image;
use rayrust::primitive;
use rayrust::ray::{ self, Intersectable };
use rayrust::types::{ Real, RealConsts };


// ------------------------
//
// ------------------------

fn diffuse(inc: &Vector3<Real>,
           _: &Vector3<Real>,
           normal: &Vector3<Real>)
//
           -> Real {
    na::dot(inc, normal)
}

fn shade(light_position: &Point3<Real>,
         sphere_center: &Point3<Real>,
         sphere_color: &color::Color,
         ray: &ray::Ray,
         intersection: Real) -> color::Color {
    let hit = *ray.origin() + *ray.direction() * intersection;
    // TODO: Това в тая посока ли трябва да е?
    let normal = na::normalize(&(hit - *sphere_center));
    let inc = na::normalize(&(*light_position - hit));
    let d = diffuse(&inc,
                    &Vector3::new(0.0, 0.0, 0.0),
                    &normal);
    if d < 0.0 {
        color::Color::default() // black
    } else {
        *sphere_color * d
    }
}


// ------------------------
//
// ------------------------

fn main() {
    let width = 800;
    let height = 600;
    let mut img = image::Image::new(width, height);

    // fn f(x: f32, y: f32) -> color::Color {
    //     color::Color::new(x, y, 0.0, 1.0)
    // }
    // stuff::texture(&mut img, f);
    // let mut file = fs::File::create("output.ppm").unwrap();
    // img.writeppm(&mut file).expect("Failed to write image file");
    // return;

    // Scene
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    // let cam = ortho::OrthographicCamera::new(width, height,
    //                                          2.0,
    //                                          &eye, &center, &up);
    // let alpha = (1.0 as Real / 5.0).atan() * 1.1;
    let cam = persp::PerspectiveCamera::new(width, height,
                                            // alpha*2.0,
                                            RealConsts::PI / 2.0,
                                            &eye, &center, &up);

    let light = Point3::new(3.0 as Real, 2.0, 5.0);
    let sphere1 = primitive::Sphere::new(&Point3::new(0 as Real, 0.0,  0.0), 1.0);
    let sphere2 = primitive::Sphere::new(&Point3::new(2 as Real, 0.0, -5.0), 1.0);

    // Colors
    let background = color::Color::new(1.0, 1.0, 1.0, 1.0);
    let color1 = color::Color::new(1.0, 0.0, 0.0, 1.0);
    let color2 = color::Color::new(0.0, 1.0, 0.0, 1.0);

    for row in 0..img.height() {
        for col in 0..img.width() {
            let ray = cam.generate_ray(col, row);

            let mut color = background;

            if let Some(t) = sphere1.intersection(&ray) {
                // color = &color1;
                color = shade(&light, sphere1.center(), &color1, &ray, t);
            } else if let Some(t) = sphere2.intersection(&ray) {
                color = shade(&light, sphere2.center(), &color2, &ray, t);
            }

            img.set(col, row, &color);
        }
    }

    let mut file = fs::File::create("output.ppm").unwrap();
    img.writeppm(&mut file).expect("Failed to write image file");
}
