extern crate nalgebra as na;

use super::lin;
use super::ray;
use super::types::{ Real };


pub struct OrthographicCamera {
    _camera_to_world: na::Matrix4<Real>,
    _raster_to_camera: na::Matrix4<Real>,
    _dim: (usize, usize),
}

impl OrthographicCamera {
    pub fn new(dim: (usize, usize)) -> OrthographicCamera {
        let ratio = (dim.0 as Real) / (dim.1 as Real);
        OrthographicCamera {
            _camera_to_world: camera_to_world(
                &lin::p(0.0, 0.0, 5.0),
                &lin::p(0.0, 0.0, 0.0),
                &lin::v(0.0, 1.0, 0.0)
            ),
            _raster_to_camera: raster_to_ndc(dim)
                * ndc_to_screen(ratio)
                * screen_to_ortho(),
            _dim: dim
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize) -> ray::Ray<Real> {
        let o1 = lin::p(x as Real, y as Real, 0.0);
        let o2 = o1 * self._raster_to_camera;
        let o3 = o2 * self._camera_to_world;

        let d1 = lin::v(0.0, 0.0, -1.0);
        let d2 = d1 * self._raster_to_camera;
        let d3 = d2 * self._camera_to_world;

        ray::Ray::new(&o3, &d3)
    }
}

pub fn raster_to_ndc(dim: (usize, usize)) -> na::Matrix4<Real> {
    let w = 1.0 / (dim.0 as Real);
    let h = 1.0 / (dim.1 as Real);
    lin::scale3f(w, h, 1.0)
}

pub fn ndc_to_screen(ratio: Real) -> na::Matrix4<Real> {
    lin::translate3f(-0.5, -0.5, 0.0) * lin::scale3f(2.0 * ratio, 2.0, 1.0)
}

pub fn screen_to_ortho() -> na::Matrix4<Real> {
    // TODO: Expand the view!
    na::Matrix4::new(
        1.0,  0.0, 0.0, 0.0,  // x
        0.0, -1.0, 0.0, 0.0,  // y
        0.0,  0.0, 1.0, 0.0,  // z
        0.0,  0.0, 0.0, 1.0,  // w
    )
}

pub fn camera_to_world(eye: &na::Point4<Real>,
                       center: &na::Point4<Real>,
                       up: &na::Vector4<Real>)
//
                       -> na::Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&lin::cross(&f, up));
    let u = lin::cross(&s, &f);
    lin::m4v(&s, &u, &-f, &eye.to_vector())
}
