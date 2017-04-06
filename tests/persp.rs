extern crate rayrust;

#[macro_use]
extern crate approx;
use approx::ApproxEq;

extern crate nalgebra as na;
use na::{ Point3, Point4, Vector3, Vector4 };

use rayrust::camera::common;
use rayrust::camera::common::{ Camera };
use rayrust::camera::persp;
use rayrust::types::{ Real, RealConsts };

#[test]
fn persp_from_screen() {
    let fovy = (60.0 as Real).to_radians();
    let tan = (fovy / 2.0).tan();
    let m = common::persp_camera_from_screen(fovy, 1.0);

    let v1 = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(m * v1, v1);

    let v2 = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let e2 = Vector4::new(0.0, tan, 0.0, 0.0);
    assert_relative_eq!(m * v2, e2);

    let v3 = Vector4::new(1.0, 1.0, -1.0, 0.0);
    let e3 = Vector4::new(tan, tan, -1.0, 0.0);
    assert_relative_eq!(m * v3, e3);

    let v4 = Vector4::new(-1.0, -1.0, -1.0, 0.0);
    let e4 = Vector4::new(-tan, -tan, -1.0, 0.0);
    assert_relative_eq!(m * v4, e4);
}

#[test]
fn camera_from_raster() {
    let width:  usize = 800;
    let height: usize = 600;
    let aspect: Real  = width as Real / height as Real;
    let fovy:   Real  = RealConsts::PI / 2.0;
    let tan:    Real  = (fovy/2.0).tan();

    let m = common::persp_camera_from_screen(fovy, aspect)
        * common::screen_from_ndc()
        * common::ndc_from_raster(width, height);

    let p1 = m * Point4::new(width as Real, 0.0, 0.0, 1.0);
    let e1 = Point4::new(tan * aspect, tan, 0.0, 1.0);
    assert_relative_eq!(p1, e1);

    let p2 = m * Point4::new(width  as Real / 2.0,
                             height as Real / 2.0,
                             0.0,
                             1.0);
    let e2 = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_relative_eq!(p2, e2);

    let p3 = m * Point4::new(0.0 as Real, height as Real, 1.0, 1.0);
    let e3 = Point4::new(-tan * aspect, -tan, 1.0, 1.0);
    assert_relative_eq!(p3, e3);
}

#[test]
fn raster_to_world() {
    let width:  usize = 800;
    let height: usize = 600;
    let aspect: Real  = width as Real / height as Real;
    let fovy:   Real  = RealConsts::PI / 2.0;
    let tan:    Real  = (fovy/2.0).tan();

    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);

    let m = common::world_from_camera(&eye, &center, &up)
        * common::persp_camera_from_screen(fovy, aspect)
        * common::screen_from_ndc()
        * common::ndc_from_raster(width, height);

    let p1 = m * Point4::new(0.0, 0.0, 0.0, 1.0);
    let e1 = Point4::new(-tan * aspect, tan, 5.0, 1.0);
    assert_relative_eq!(p1, e1);
}

#[test]
fn perspective_camera() {
    let width:  usize = 800;
    let height: usize = 600;
    let aspect: Real  = width as Real / height as Real;
    let fovy:   Real  = RealConsts::PI / 2.0;
    let tan:    Real  = (fovy/2.0).tan();

    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);

    let cam = persp::PerspectiveCamera::new(width, height,
                                            fovy,
                                            &eye, &center, &up);

    let r1 = cam.generate_ray(width/2, height/2);
    let e1 = Vector3::new(0 as Real, 0.0, -1.0);
    assert_relative_eq!(*r1.origin(), eye);
    assert_relative_eq!(*r1.direction(), e1);

    let r2 = cam.generate_ray(width, height);
    let e2 = na::normalize(&Vector3::new(tan * aspect, -tan, -1.0));
    assert_relative_eq!(*r2.origin(), eye);
    assert_relative_eq!(*r2.direction(), e2);
}
