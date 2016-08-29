extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq };

use rayrust::camera;
use rayrust::linear;
use rayrust::types::{ Real, RealConsts };


fn p(x: Real, y: Real, z: Real) -> na::Point4<Real> {
    na::Point4::<Real>::new(x, y, z, 1.0)
}

fn v(x: Real, y: Real, z: Real) -> na::Vector4<Real> {
    na::Vector4::<Real>::new(x, y, z, 0.0)
}

#[test]
fn camera_to_world() {
    let eye = p(0.0, 0.0, 5.0);
    let center = na::Point3::<Real>::new(0.0, 0.0, 0.0);
    let up = na::Vector3::<Real>::new(0.0, 1.0, 0.0);
    let M = camera::camera_to_world(&eye, &center, &up);
    assert_approx_eq!(M, na::Matrix4::new(
        1.0, 0.0,  0.0, 0.0,  // x
        0.0, 1.0,  0.0, 0.0,  // y
        0.0, 0.0,  1.0, 0.0,  // z
        0.0, 0.0, -5.0, 1.0,  // t
    ));
}

#[test]
fn camera_to_world1() {
    let eye = na::Point3::<Real>::new(5.0, 5.0, 0.0);
    let center = na::Point3::<Real>::new(0.0, 0.0, 0.0);
    let up = na::Vector3::<Real>::new(0.0, 1.0, 0.0);
    let M = camera::camera_to_world(&eye, &center, &up);

    let s = (RealConsts::PI / 4.0).sin();
    assert_approx_eq!(M, na::Matrix4::new(
         0.0,  0.0, -1.0, 0.0,  // x
          -s,    s,  0.0, 0.0,  // y
           s,    s,  0.0, 0.0,  // z
        -5.0, -5.0,  0.0, 1.0   // t
    ));
}

#[test]
fn ndc_to_screen() {
    let M = camera::ndc_to_screen(2.0);

    let u = v(0.0, 0.0, 0.0);
    assert_approx_eq!(u*M, v(-2.0, -1.0, 0.0));

    let p = v(0.5, 0.5, 0.0);
    assert_approx_eq!(p*M, v(0.0, 0.0, 0.0));

    let r = v(1.0, 1.0, 0.0);
    assert_approx_eq!(r*M, v(2.0, 1.0, 0.0));
}

#[test]
fn raster_to_ndc() {
    let M = camera::raster_to_ndc((800, 600));

    let u = v(0.0, 0.0, 0.0);
    assert_approx_eq!(u*M, v(0.0, 0.0, 0.0));

    let p = v(400.0, 300.0, 0.0);
    assert_approx_eq!(p*M, v(0.5, 0.5, 0.0));

    let r = v(800.0, 600.0, 0.0);
    assert_approx_eq!(r*M, v(1.0, 1.0, 0.0));
}

#[test]
fn raster_to_screen() {
    let M = camera::raster_to_ndc((800, 400)) * camera::ndc_to_screen(2.0);

    let u = v(0.0, 0.0, 0.0);
    assert_approx_eq!(u*M, v(-2.0, -1.0, 0.0));

    let p = v(400.0, 200.0, 0.0);
    assert_approx_eq!(p*M, v(0.0, 0.0, 0.0));

    let r = v(800.0, 400.0, 0.0);
    assert_approx_eq!(r*M, v(2.0, 1.0, 0.0));
}
