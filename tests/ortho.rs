extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq, Point3, Point4, Vector3 };

use rayrust::camera::ortho;
use rayrust::types::{ RealConsts };


// ------------------------
// Functions
// ------------------------

#[test]
fn camera_to_world_0() {
    let eye = Point3::new(0.0, 0.0, 5.0);
    let center = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let m = ortho::camera_to_world(&eye, &center, &up);
    assert_approx_eq!(m, na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,  // x
        0.0, 1.0, 0.0, 0.0,  // y
        0.0, 0.0, 1.0, 0.0,  // z
        0.0, 0.0, 5.0, 1.0,  // t
    ));
}

#[test]
fn camera_to_world_1() {
    let eye = Point3::new(5.0, 5.0, 0.0);
    let center = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let m = ortho::camera_to_world(&eye, &center, &up);

    let s = (RealConsts::PI / 4.0).sin();
    assert_approx_eq!(m, na::Matrix4::new(
        0.0, 0.0, -1.0, 0.0,  // x
         -s,   s,  0.0, 0.0,  // y
          s,   s,  0.0, 0.0,  // z
        5.0, 5.0,  0.0, 1.0   // t
    ));
}

#[test]
fn ndc_to_screen() {
    let m = ortho::ndc_to_screen(2.0);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(u*m, Point4::new(-2.0, 1.0, 0.0, 1.0));

    let p = Point4::new(0.5, 0.5, 0.0, 1.0);
    assert_approx_eq!(p*m, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(1.0, 1.0, 0.0, 1.0);
    assert_approx_eq!(r*m, Point4::new(2.0, -1.0, 0.0, 1.0));
}

#[test]
fn raster_to_ndc() {
    let m = ortho::raster_to_ndc(800, 600);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(u*m, Point4::new(0.0, 0.0, 0.0, 1.0));

    let p = Point4::new(400.0, 300.0, 0.0, 1.0);
    assert_approx_eq!(p*m, Point4::new(0.5, 0.5, 0.0, 1.0));

    let r = Point4::new(800.0, 600.0, 0.0, 1.0);
    assert_approx_eq!(r*m, Point4::new(1.0, 1.0, 0.0, 1.0));
}

#[test]
fn raster_to_screen() {
    let width = 800;
    let height = 400;
    let aspect = 800 as f32 / height as f32;
    let m = ortho::raster_to_ndc(width, height) * ortho::ndc_to_screen(aspect);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(u*m, Point4::new(-2.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_approx_eq!(p*m, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_approx_eq!(r*m, Point4::new(2.0, -1.0, 0.0, 1.0));
}

#[test]
fn raster_to_ortho() {
    let m = ortho::raster_to_ndc(800, 400)
        * ortho::ndc_to_screen(2.0)
        * ortho::screen_to_ortho();

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(u*m, Point4::new(-2.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_approx_eq!(p*m, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_approx_eq!(r*m, Point4::new(2.0, -1.0, 0.0, 1.0));    
}


// ------------------------
// OrthographicCamera
// ------------------------

#[test]
fn orthographic_camera_0() {
    let cam = ortho::OrthographicCamera::new(800, 400);
    let gen = cam.generate_ray(400, 200);
    assert_approx_eq!(*gen.origin(), Point3::new(0.0, 0.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_1() {
    let cam = ortho::OrthographicCamera::new(800, 400);
    let gen = cam.generate_ray(0, 0);
    assert_approx_eq!(*gen.origin(), Point3::new(-2.0, 1.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_2() {
    let cam = ortho::OrthographicCamera::new(800, 400);
    let gen = cam.generate_ray(800, 400);
    assert_approx_eq!(*gen.origin(), Point3::new(2.0, -1.0, 5.0));
    assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
}
