use na;
use na::{ Matrix4, Point3, Vector3 };

use geometry::lin;
use geometry::ray::{ Ray };
use types::{ Real };


pub const NEAR: Real = -1.0;


pub trait Camera {
    fn generate_ray(&self, x: usize, y: usize) -> Ray;
    // TODO: fn generate_ray_differential() -> x
}

/// Normalized device coordinates:
/// x ∈ [0; 1]
/// y ∈ [0; 1]
/// Center (0, 0) lies in the top-left corner.
pub fn raster_to_ndc(width: usize, height: usize) -> Matrix4<Real> {
    let w = 1.0 / (width as Real);
    let h = 1.0 / (height as Real);
    lin::scale2f(w, h)
}

/// Screen space:
/// x ∈ [-1; 1]
/// y ∈ [-1; 1]
/// Center (0, 0) lies in the middle (both horizontal and vertical).
pub fn ndc_to_screen() -> Matrix4<Real> {
    lin::scale2f(2.0, -2.0) * lin::translate2f(-0.5, -0.5)
}

/// Converts screen coordinates to orthographic camera coordinates.
pub fn screen_to_ortho_camera(plane: Real, aspect: Real) -> Matrix4<Real> {
    let y: Real = plane / 2.0;
    let x: Real = y * aspect;
    lin::scale2f(x, y)
}

pub fn screen_to_persp_camera(fovy: Real, aspect: Real) -> Matrix4<Real> {
    let n = NEAR.abs();             // Near
    let t = (fovy / 2.0).tan() * n; // Top
    let r = t * aspect;             // Right
    lin::scale2f(r, t)
}

/// The standard look-at function.  Shamelessly copied from OpenGL's
/// documentation.
pub fn camera_to_world(eye:    &Point3<Real>,
                       center: &Point3<Real>,
                       up:     &Vector3<Real>) -> Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    // let s = na::normalize(&na::cross(&f, up));
    let s = na::normalize(&(f.cross(&up)));
    // let u = na::cross(&s, &f);
    let u = s.cross(&f);
    lin::m4v(&s, &u, &-f, &eye)
}
