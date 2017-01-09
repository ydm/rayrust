extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq, Point3, Vector3 };

use rayrust::camera::common::{ Camera };
use rayrust::camera::ortho;
use rayrust::types::{ Real };


// ------------------------
// OrthographicCamera
// ------------------------

#[test]
fn orthographic_camera_0() {
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);

    let cam = ortho::OrthographicCamera::new(800, 400, 2.0, &eye, &center, &up);
    let gen = cam.generate_ray(400, 200);

    assert_approx_eq!(*gen.origin(), Point3::new(0.0, 0.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_1() {
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    
    let cam = ortho::OrthographicCamera::new(800, 400, 2.0, &eye, &center, &up);
    let gen = cam.generate_ray(0, 0);

    assert_approx_eq!(*gen.origin(), Point3::new(-2.0, 1.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_2() {
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);

    let cam = ortho::OrthographicCamera::new(800, 400, 2.0, &eye, &center, &up);
    let gen = cam.generate_ray(800, 400);

    assert_approx_eq!(*gen.origin(), Point3::new(2.0, -1.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}
