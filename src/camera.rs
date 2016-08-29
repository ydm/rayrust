extern crate nalgebra as na;

// use self::na::FromHomogeneous;

use super::linear;
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
                &na::Point3::new(0.0, 0.0, 5.0),
                &na::Point3::new(0.0, 0.0, 0.0),
                &na::Vector3::new(0.0, 1.0, 0.0)
            ),
            _raster_to_camera: raster_to_ndc(dim)
                * ndc_to_screen(ratio)
                * screen_to_ortho(),
            _dim: dim
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize) -> ray::Ray<Real> {
        let o1 = na::Point4::<Real>::new(x as Real, y as Real, 0.0, 1.0);
        let o2 = o1 * self._raster_to_camera;
        let o3 = o2 * self._camera_to_world;

        let d1 = na::Vector4::<Real>::new(0.0, 0.0, -1.0, 0.0);
        let d2 = d1 * self._raster_to_camera;
        let d3 = d2 * self._camera_to_world;

        ray::Ray::new(
            &linear::p3_from_p4(&o3),
            &linear::v3_from_v4(&d3)
        )
    }
}

pub fn raster_to_ndc(dim: (usize, usize)) -> na::Matrix4<Real> {
    let w = 1.0 / (dim.0 as Real);
    let h = 1.0 / (dim.1 as Real);
    linear::scale3f(w, h, 1.0)
}

pub fn ndc_to_screen(ratio: Real) -> na::Matrix4<Real> {
    linear::translate3f(-0.5, -0.5, 0.0)
        * linear::scale3f(2.0 * ratio, 2.0, 1.0)
}

pub fn screen_to_ortho() -> na::Matrix4<Real> {
    na::new_identity(4)
}

pub fn camera_to_world(eye: &na::Point3<Real>,
                       center: &na::Point3<Real>,
                       up: &na::Vector3<Real>)
//
                       -> na::Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&na::cross(&f, up));
    let u = na::cross(&s, &f);
    let t = linear::v3_from_p3(eye);
    linear::m4_from_4v3(&s, &u, &-f, &-t)
}

// pub fn raster_to_ndc(width: u16, height: u16, p: &na::Vector3<u16>)
//                      -> na::Vector3<f32> {
//     let w = 1.0 / (w as f32);
//     let h = 1.0 / (h as f32);
//     let o = 1.0;
//     let z = 0.0;
//     let scale = na::Matrix3::new(
//         w, z, z,
//         z, h, z,
//         z, z, o,
//     );
//     let v = &na::Vector3::new(
//         p.x as f32,
//         p.y as f32,
//         0.0
//     );

//     scale(1
//     // let raster_to_ndc = na::Matrix4
//     let x = (pras.x as f32) / (w as f32);
//     let y = (pras.y as f32) / (h as f32);
//     na::Vector3::new(x, y, 0.0)
// }

// // pub fn ndc_to_
