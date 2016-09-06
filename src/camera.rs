extern crate nalgebra as na;

use self::na::{ Point3, Vector3 };
use super::lin;
use super::ray;
use super::types::{ Real };


pub struct OrthographicCamera {
    _camera_to_world: na::Matrix4<Real>,
    _raster_to_camera: na::Matrix4<Real>,
}

impl OrthographicCamera {
    pub fn new(width: usize, height: usize) -> OrthographicCamera {
        let ratio = (width as Real) / (height as Real);
        OrthographicCamera {
            _camera_to_world: camera_to_world(
                &Point3::new(0.0, 0.0, 5.0),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0)
            ),
            _raster_to_camera: raster_to_ndc(width, height)
                * ndc_to_screen(ratio)
                * screen_to_ortho(),
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize) -> ray::Ray<Real> {
        let origin = na::Point4::new(x as Real, y as Real, 0.0, 1.0)
            * self._raster_to_camera
            * self._camera_to_world;

        let direction = na::Vector4::new(0.0 as Real, 0.0, -1.0, 0.0)
            * self._raster_to_camera
            * self._camera_to_world;

        ray::Ray::new(&na::from_homogeneous(&origin),
                      &na::from_homogeneous(&direction))
    }
}

pub fn raster_to_ndc(width: usize, height: usize) -> na::Matrix4<Real> {
    let w = 1.0 / (width as Real);
    let h = 1.0 / (height as Real);
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

pub fn camera_to_world(eye:    &Point3<Real>,
                       center: &Point3<Real>,
                       up:     &Vector3<Real>)
//
                       -> na::Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&na::cross(&f, up));
    let u = na::cross(&s, &f);
    lin::m4v(&s, &u, &-f, &eye.to_vector())
}
