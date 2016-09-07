use na;
use na::{ Matrix4, Point3, Point4, Vector3, Vector4 };

use lin;
use ray;
use types::{ Real };


pub struct OrthographicCamera {
    _raster_to_camera: Matrix4<Real>,
    _camera_to_world: Matrix4<Real>,
}

impl OrthographicCamera {
    pub fn new(width: usize, height: usize) -> OrthographicCamera {
        let ratio = (width as Real) / (height as Real);
        OrthographicCamera {
            _raster_to_camera: raster_to_ndc(width, height)
                * ndc_to_screen(ratio)
                * screen_to_ortho(),
            _camera_to_world: camera_to_world(
                &Point3::new(0.0, 0.0, 5.0),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0)
            ),
        }
    }

    pub fn generate_ray(&self, x: usize, y: usize) -> ray::Ray<Real> {
        let origin = Point4::new(x as Real, y as Real, 0.0, 1.0)
            * self._raster_to_camera
            * self._camera_to_world;

        let direction = Vector4::new(0.0 as Real, 0.0, -1.0, 0.0)
            * self._raster_to_camera
            * self._camera_to_world;

        ray::Ray::new(&na::from_homogeneous(&origin),
                      &na::from_homogeneous(&direction))
    }
}

pub fn raster_to_ndc(width: usize, height: usize) -> Matrix4<Real> {
    let w = 1.0 / (width as Real);
    let h = 1.0 / (height as Real);
    lin::scale3f(w, h, 1.0)
}

pub fn ndc_to_screen(ratio: Real) -> Matrix4<Real> {
    lin::translate3f(-0.5, -0.5, 0.0) * lin::scale3f(2.0 * ratio, -2.0, 1.0)
}

pub fn screen_to_ortho() -> Matrix4<Real> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,  // x
        0.0, 1.0, 0.0, 0.0,  // y
        0.0, 0.0, 1.0, 0.0,  // z
        0.0, 0.0, 0.0, 1.0,  // w
    )
}

pub fn camera_to_world(eye:    &Point3<Real>,
                       center: &Point3<Real>,
                       up:     &Vector3<Real>)
//
                       -> Matrix4<Real> {
    let d = *center - *eye;
    let f = na::normalize(&d);
    let s = na::normalize(&na::cross(&f, up));
    let u = na::cross(&s, &f);
    lin::m4v(&s, &u, &-f, &eye.to_vector())
}
