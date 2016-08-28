extern crate nalgebra as na;

// use self::na::ToHomogeneous;

use super::linear;
use super::ray;
use super::types::{ Real };


struct OrthographicCamera {
    _camera_to_world: na::Matrix4<Real>,
    _raster_to_camera: na::Matrix4<Real>,
    _dim: (usize, usize),
}

impl OrthographicCamera {
    pub fn new(dim: (usize, usize)) -> OrthographicCamera {
        let ratio = (dim.0 as Real) / (dim.1 as Real);
        OrthographicCamera {
            _camera_to_world: camera_to_world(
                &na::Vector3::new(0.0, 0.0, 5.0),
                &na::Vector3::new(0.0, 0.0, 0.0),
                &na::Vector3::new(0.0, 1.0, 0.0)
            ),
            _raster_to_camera: raster_to_ndc(dim)
                * ndc_to_screen(ratio)
                * screen_to_ortho(),
            _dim: dim
        }
    }

    // pub fn generate_ray(&self, x: usize, y: usize) -> ray::Ray {
    //     // na::Vector3::<Real>::new(x as Real, y as Real, 1.0)
    // }
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

pub fn camera_to_world(eye: &na::Vector3<Real>,
                       center: &na::Vector3<Real>,
                       up: &na::Vector3<Real>)
                       -> na::Matrix4<Real> {
    //
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&na::cross(&f, up));
    let u = na::cross(&f, &s);
    linear::from4fv(&s, &u, &-f, &-*eye)
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
