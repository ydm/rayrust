extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq, Point3, Point4, Vector3, Matrix4 };

use rayrust::camera::ortho;
use rayrust::types::{ Real, RealConsts };
use rayrust::lin;


// ------------------------
// Functions
// ------------------------

#[test]
fn camera_to_world_0() {
    let eye    = Point3 ::new(0.0 as Real, 0.0, 5.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    let m = ortho::camera_to_world(&eye, &center, &up);

    let x = Vector3::new(1.0, 0.0, 0.0);
    let y = Vector3::new(0.0, 1.0, 0.0);
    let z = Vector3::new(0.0, 0.0, 1.0);
    let p = Vector3::new(0.0, 0.0, 5.0);
    assert_approx_eq!(m, lin::m4v(&x, &y, &z, &p));
}

#[test]
fn camera_to_world_1() {
    let eye    = Point3 ::new(5.0 as Real, 5.0, 0.0);
    let center = Point3 ::new(0.0 as Real, 0.0, 0.0);
    let up     = Vector3::new(0.0 as Real, 1.0, 0.0);
    let m = ortho::camera_to_world(&eye, &center, &up);

    let s = (RealConsts::PI / 4.0).sin();
    let x = Vector3::new(0.0, 0.0, -1.0);
    let y = Vector3::new( -s,   s,  0.0);
    let z = Vector3::new(  s,   s,  0.0);
    let p = Vector3::new(5.0, 5.0,  0.0);
    assert_approx_eq!(m, lin::m4v(&x, &y, &z, &p));
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
fn ndc_to_screen() {
    let m = ortho::ndc_to_screen();

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(m*u, Point4::new(-1.0, 1.0, 0.0, 1.0));

    let p = Point4::new(0.5, 0.5, 0.0, 1.0);
    assert_approx_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(1.0, 1.0, 0.0, 1.0);
    assert_approx_eq!(m*r, Point4::new(1.0, -1.0, 0.0, 1.0));
}

#[test]
fn raster_to_screen() {
    let width = 800;
    let height = 400;
    let aspect = width as Real / height as Real;
    let m = ortho::ndc_to_screen() * ortho::raster_to_ndc(width, height);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(m*u, Point4::new(-1.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_approx_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_approx_eq!(m*r, Point4::new(1.0, -1.0, 0.0, 1.0));
}

#[test]
fn raster_to_ortho_camera() {
    let width = 800;
    let height = 400;
    let aspect = width as Real / height as Real;

    let m = ortho::screen_to_ortho_camera(2.0, aspect)
        * ortho::ndc_to_screen()
        * ortho::raster_to_ndc(width, height);

    let u = Point4::new(0.0, 0.0, 0.0, 1.0);
    assert_approx_eq!(m*u, Point4::new(-2.0, 1.0, 0.0, 1.0));

    let p = Point4::new(400.0, 200.0, 0.0, 1.0);
    assert_approx_eq!(m*p, Point4::new(0.0, 0.0, 0.0, 1.0));

    let r = Point4::new(800.0, 400.0, 0.0, 1.0);
    assert_approx_eq!(m*r, Point4::new(2.0, -1.0, 0.0, 1.0));    
}


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
