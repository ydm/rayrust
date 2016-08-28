extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq };

use rayrust::camera;
use rayrust::types::{ Real };


fn v(x: Real, y: Real, z: Real) -> na::Vector4<Real> {
    na::Vector4::<Real>::new(x, y, z, 1.0)
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
fn raster_to_screen() {
    let M = camera::raster_to_ndc((800, 400)) * camera::ndc_to_screen(2.0);

    let u = v(0.0, 0.0, 0.0);
    assert_approx_eq!(u*M, v(-2.0, -1.0, 0.0));

    let p = v(400.0, 200.0, 0.0);
    assert_approx_eq!(p*M, v(0.0, 0.0, 0.0));

    let r = v(800.0, 400.0, 0.0);
    assert_approx_eq!(r*M, v(2.0, 1.0, 0.0));
}
