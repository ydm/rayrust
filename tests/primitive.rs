extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq };

use rayrust::primitive;
use rayrust::ray::{ self, Intersectable };


#[test]
fn test_sphere_intersection() {
    let sphere = primitive::Sphere::new(
        &na::Vector3::new(0.0, 0.0, 0.0), // center
        1.0                               // radius
    );
    let ray = ray::Ray::<f32>::new(
        &na::Vector3::new(0.0, 0.0,  5.0),
        &na::Vector3::new(0.0, 0.0, -1.0)
    );

    let ts = sphere.intersections(&ray);
    assert_approx_eq!(ts[0], 4.0);
    assert_approx_eq!(ts[1], 6.0);
}

