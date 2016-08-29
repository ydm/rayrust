extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq };

use rayrust::camera;
use rayrust::lin;
use rayrust::types::{ RealConsts };


// ------------------------
// Functions
// ------------------------

#[test]
fn camera_to_world() {
    let eye = lin::p(0.0, 0.0, 5.0);
    let center = lin::p(0.0, 0.0, 0.0);
    let up = lin::v(0.0, 1.0, 0.0);
    let m = camera::camera_to_world(&eye, &center, &up);
    assert_approx_eq!(m, na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,  // x
        0.0, 1.0, 0.0, 0.0,  // y
        0.0, 0.0, 1.0, 0.0,  // z
        0.0, 0.0, 5.0, 1.0,  // t
    ));
}

#[test]
fn camera_to_world1() {
    let eye = lin::p(5.0, 5.0, 0.0);
    let center = lin::p(0.0, 0.0, 0.0);
    let up = lin::v(0.0, 1.0, 0.0);
    let m = camera::camera_to_world(&eye, &center, &up);

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
    let m = camera::ndc_to_screen(2.0);

    let u = lin::p(0.0, 0.0, 0.0);
    assert_approx_eq!(u*m, lin::p(-2.0, -1.0, 0.0));

    let p = lin::p(0.5, 0.5, 0.0);
    assert_approx_eq!(p*m, lin::p(0.0, 0.0, 0.0));

    let r = lin::p(1.0, 1.0, 0.0);
    assert_approx_eq!(r*m, lin::p(2.0, 1.0, 0.0));
}

#[test]
fn raster_to_ndc() {
    let m = camera::raster_to_ndc((800, 600));

    let u = lin::p(0.0, 0.0, 0.0);
    assert_approx_eq!(u*m, lin::p(0.0, 0.0, 0.0));

    let p = lin::p(400.0, 300.0, 0.0);
    assert_approx_eq!(p*m, lin::p(0.5, 0.5, 0.0));

    let r = lin::p(800.0, 600.0, 0.0);
    assert_approx_eq!(r*m, lin::p(1.0, 1.0, 0.0));
}

#[test]
fn raster_to_screen() {
    let m = camera::raster_to_ndc((800, 400)) * camera::ndc_to_screen(2.0);

    let u = lin::p(0.0, 0.0, 0.0);
    assert_approx_eq!(u*m, lin::p(-2.0, -1.0, 0.0));

    let p = lin::p(400.0, 200.0, 0.0);
    assert_approx_eq!(p*m, lin::p(0.0, 0.0, 0.0));

    let r = lin::p(800.0, 400.0, 0.0);
    assert_approx_eq!(r*m, lin::p(2.0, 1.0, 0.0));
}

#[test]
fn raster_to_ortho() {
    let m = camera::raster_to_ndc((800, 400))
        * camera::ndc_to_screen(2.0)
        * camera::screen_to_ortho();

    let u = lin::p(0.0, 0.0, 0.0);
    assert_approx_eq!(u*m, lin::p(-2.0, 1.0, 0.0));

    let p = lin::p(400.0, 200.0, 0.0);
    assert_approx_eq!(p*m, lin::p(0.0, 0.0, 0.0));

    let r = lin::p(800.0, 400.0, 0.0);
    assert_approx_eq!(r*m, lin::p(2.0, -1.0, 0.0));    
}


// ------------------------
// OrthographicCamera
// ------------------------

#[test]
fn orthographic_camera_0() {
    let cam = camera::OrthographicCamera::new((800, 400));
    let gen = cam.generate_ray(400, 200);
    assert_approx_eq!(*gen.origin(), lin::p(0.0, 0.0, 5.0));
    assert_approx_eq!(*gen.direction(), lin::v(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_1() {
    let cam = camera::OrthographicCamera::new((800, 400));
    let gen = cam.generate_ray(0, 0);
    assert_approx_eq!(*gen.origin(), lin::p(-2.0, 1.0, 5.0));
    assert_approx_eq!(*gen.direction(), lin::v(0.0, 0.0, -1.0));
}

#[test]
fn orthographic_camera_2() {
    let cam = camera::OrthographicCamera::new((800, 400));
    let gen = cam.generate_ray(800, 400);
    assert_approx_eq!(*gen.origin(), lin::p(2.0, -1.0, 5.0));
    assert_approx_eq!(*gen.direction(), lin::v(0.0, 0.0, -1.0));
}
