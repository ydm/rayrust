extern crate rayrust;

// #[macro_use(assert_relative_eq)]
#[macro_use]
extern crate approx;
use approx::ApproxEq;

extern crate nalgebra as na;
use na::{ Point3, Point4, Vector3 };

use rayrust::camera::common;
use rayrust::types::{ Real, RealConsts };
use rayrust::geometry::lin;


// ------------------------
// Functions
// ------------------------

#[test]
fn world_from_camera_0() {
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    let m = common::world_from_camera(&eye, &center, &up);

    let x = Vector3::new(1.0, 0.0, 0.0);
    let y = Vector3::new(0.0, 1.0, 0.0);
    let z = Vector3::new(0.0, 0.0, 1.0);
    let p = Vector3::new(0.0, 0.0, 5.0);
    assert_relative_eq!(m, lin::m4v(&x, &y, &z, &p));
}

#[test]
fn world_from_camera_1() {
    let eye    = Point3 ::new(5.0 as Real, 5.0, 0.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    let m = common::world_from_camera(&eye, &center, &up);

    let s = (RealConsts::PI / 4.0).sin();
    let x = Vector3::new(0.0, 0.0, -1.0);
    let y = Vector3::new( -s,   s,  0.0);
    let z = Vector3::new(  s,   s,  0.0);
    let p = Vector3::new(5.0, 5.0,  0.0);
    assert_relative_eq!(m, lin::m4v(&x, &y, &z, &p));
}

#[test]
fn ndc_from_raster() {
    let m = common::ndc_from_raster(800, 600);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(u*m, Vector4::new(0.0, 0.0, 0.0, 1.0));

    let p = Point4::new(400.0, 300.0, 0.0, 1.0);
    assert_relative_eq!(p*m, Vector4::new(0.5, 0.5, 0.0, 1.0));

    let r = Point4::new(800.0, 600.0, 0.0, 1.0);
    assert_relative_eq!(r*m, Vector4::new(1.0, 1.0, 0.0, 1.0));
}

#[test]
fn ndc_from_screen() {
    let m = common::screen_from_ndc();

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(m*u, Point4::new(-1.0, 1.0, 0.0, 1.0));

    let p = Point4::new(0.5, 0.5, 0.0, 1.0);
    assert_relative_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(1.0, 1.0, 0.0, 1.0);
    assert_relative_eq!(m*r, Point4::new(1.0, -1.0, 0.0, 1.0));
}

#[test]
fn screen_from_raster() {
    let width = 800;
    let height = 400;
    let m = common::screen_from_ndc() * common::ndc_from_raster(width, height);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(m*u, Point4::new(-1.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_relative_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_relative_eq!(m*r, Point4::new(1.0, -1.0, 0.0, 1.0));
}

#[test]
fn ortho_camera_from_raster() {
    let width = 800;
    let height = 400;
    let aspect = width as Real / height as Real;

    let m = common::ortho_camera_from_screen(2.0, aspect)
        * common::screen_from_ndc()
        * common::ndc_from_raster(width, height);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(m*u, Point4::new(-2.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_relative_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_relative_eq!(m*r, Point4::new(2.0, -1.0, 0.0, 1.0));    
}
