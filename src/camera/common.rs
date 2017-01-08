use na;
use na::{ Matrix4, Point3, Vector3 };

use lin;
use ray;
use types::{ Real };


pub trait Camera {
    fn generate_ray(&self, x: usize, y: usize) -> ray::Ray;
    // TODO: fn generate_ray_differential()
}

/// Normalized device coordinates:
/// x ∈ [0; 1]
/// y ∈ [0; 1]
/// Center (0, 0) lies in the top-left corner.
pub fn raster_to_ndc(width: usize, height: usize) -> Matrix4<Real> {
    let w = 1.0 / (width as Real);
    let h = 1.0 / (height as Real);
    lin::scale3f(w, h, 1.0)
}

/// Screen space:
/// x ∈ [-1; 1]
/// y ∈ [-1; 1]
/// Center (0, 0) lies in the middle (both horizontal and vertical).
pub fn ndc_to_screen() -> Matrix4<Real> {
    lin::scale3f(2.0, -2.0, 1.0) * lin::translate3f(-0.5, -0.5, 0.0)
}

/// Converts screen coordinates to orthographic camera coordinates.
pub fn screen_to_ortho_camera(plane: Real, aspect: Real) -> Matrix4<Real> {
    let y: Real = plane / 2.0;
    let x: Real = aspect * y;
    let o: Real = 0.0;
    let l: Real = 1.0;
    //  x, y, z, p
    Matrix4::new(
        x, o, o, o,
        o, y, o, o,
        o, o, l, o,
        o, o, o, l,
    )
}

/// The standard look-at function.  Shamelessly copied from OpenGL's
/// documentation.
pub fn camera_to_world(eye:    &Point3<Real>,
                       center: &Point3<Real>,
                       up:     &Vector3<Real>) -> Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&na::cross(&f, up));
    let u = na::cross(&s, &f);
    lin::m4v(&s, &u, &-f, &eye.to_vector())
}
