extern crate nalgebra as na;
extern crate rayrust;

use std::fs;
use std::borrow::Borrow;

use na::{ Point3, Vector3 };

// use rayrust::cameras::ortho;
use rayrust::cameras::persp::PerspectiveCamera;
use rayrust::core::aggregate::LinearAggregate;
use rayrust::core::camera::Camera;
use rayrust::core::integrator::{ Integrator, SamplerIntegrator };
use rayrust::core::interaction::Intersectable;
use rayrust::core::primitive::Primitive;
use rayrust::core::scene::Scene;
use rayrust::core::types::{ Real, RealConsts };
use rayrust::shapes::sphere::Sphere;


// ------------------------
//
// ------------------------

// fn diffuse(inc: &Vector3<Real>,
//            _: &Vector3<Real>,
//            normal: &Vector3<Real>)
// //
//            -> Real {
//     na::dot(inc, normal)
// }

// fn shade(light_position: &Point3<Real>,
//          sphere_center: &Point3<Real>,
//          sphere_color: &Color,
//          ray: &Ray,
//          intersection: Real) -> Color {
//     let hit = *ray.origin() + *ray.direction() * intersection;
//     // TODO: Това в тая посока ли трябва да е?
//     let normal = na::normalize(&(hit - *sphere_center));
//     let inc = na::normalize(&(*light_position - hit));
//     let d = diffuse(&inc,
//                     &Vector3::new(0.0, 0.0, 0.0),
//                     &normal);
//     if d < 0.0 {
//         Color::default() // black
//     } else {
//         *sphere_color * d
//     }
// }


// ------------------------
// RenderOptions::MakeScene (api.cpp)
// ------------------------

fn make_aggregate(v: Vec<Primitive>) -> Box<Intersectable> {
    Box::new(LinearAggregate::new(v))
}

fn make_camera() -> Box<Camera> {
    // Definitely not Copy&Paste!
    let width = 800;
    let height = 600;

    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    // let cam = ortho::OrthographicCamera::new(width, height,
    //                                          2.0,
    //                                          &eye, &center, &up);
    Box::new(PerspectiveCamera::new(
        width, height, RealConsts::PI / 2.0, &eye, &center, &up
    ))
}

fn make_primitives() -> Vec<Primitive> {
    vec![
        Primitive::new(Box::new(Sphere::new(&Point3::origin(), 1.0))),
        Primitive::new(Box::new(Sphere::new(&Point3::new(2.0, 0.0, -3.0), 1.0))),
    ]
}

fn make_scene() -> Box<Scene> {
    let prims = make_primitives();
    let aggregate = make_aggregate(prims);
    Box::new(Scene::new(aggregate))
}


// ------------------------
// Main
// ------------------------

fn main() {
    let scene = make_scene();
    let camera = make_camera();
    let integrator = SamplerIntegrator::new();
    let img = integrator.render(scene.borrow(), camera.borrow());
    let mut file = fs::File::create("output.ppm").unwrap();
    img.writeppm(&mut file).expect("Failed to write image file");
}

// fn main0() {
//     let width = 800;
//     let height = 600;
//     let mut img = image::Image::new(width, height);

//     // fn f(x: f32, y: f32) -> Color {
//     //     Color::new(x, y, 0.0, 1.0)
//     // }
//     // stuff::texture(&mut img, f);
//     // let mut file = fs::File::create("output.ppm").unwrap();
//     // img.writeppm(&mut file).expect("Failed to write image file");
//     // return;

//     // Scene

//     let light = Point3::new(3.0 as Real, 2.0, 5.0);
//     let sphere1 = Sphere::new(&Point3::new(0.0 as Real, 0.0,  0.0), 1.0);
//     let sphere2 = Sphere::new(&Point3::new(2.0 as Real, 0.0, -3.0), 1.0);

//     // Colors
//     let background = Color::new(1.0, 1.0, 1.0, 1.0);
//     let color1 = Color::new(1.0, 0.0, 0.0, 1.0);
//     let color2 = Color::new(0.0, 1.0, 0.0, 1.0);

//     for row in 0..img.height() {
//         for col in 0..img.width() {
//             let ray = cam.generate_ray(col, row);

//             let mut color = background;

//             if let Some(t) = sphere1.intersect(&ray) {
//                 // color = &color1;
//                 color = shade(&light, sphere1.center(), &color1, &ray, t);
//             } else if let Some(t) = sphere2.intersect(&ray) {
//                 color = shade(&light, sphere2.center(), &color2, &ray, t);
//             }

//             img.set(col, row, &color);
//         }
//     }

//     let mut file = fs::File::create("output.ppm").unwrap();
//     img.writeppm(&mut file).expect("Failed to write image file");
// }
