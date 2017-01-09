use na;
use na::{ Matrix4, Point3, Point4, Vector3 };

use camera::common;
use ray::{ Ray };
use types::{ Real };


pub struct PerspectiveCamera {
    _raster_to_world: Matrix4<Real>,
    _eye: Point3<Real>,
}

impl PerspectiveCamera {
    /// Create a perspective camera
    ///
    /// * `width` - Image width
    /// * `height` - Image height
    /// * `fovy` - Field of view angle, in radians, in the y direction
    /// * `eye` - Camera's location
    /// * `center` - Where the camera is looking at
    /// * `up` - Camera's up vector
    pub fn new(width: usize,
               height: usize,
               fovy: Real,
               eye: &Point3<Real>,
               center: &Point3<Real>,
               up: &Vector3<Real>) -> PerspectiveCamera {
        //
        let aspect = (width as Real) / (height as Real);
        PerspectiveCamera {
            _raster_to_world: common::camera_to_world(&eye, &center, &up)
                * common::screen_to_persp_camera(fovy, aspect)
                * common::ndc_to_screen()
                * common::raster_to_ndc(width, height),
            _eye: *eye,
        }
    }
}

impl common::Camera for PerspectiveCamera {
    fn generate_ray(&self, x: usize, y: usize) -> Ray {
        // The image point (x, y), projected onto the near clipping
        // plane (near == -1)
        let  n = Point4::new(x as Real, y as Real, -1.0, 1.0);
        let w4: Point4<Real> = self._raster_to_world * n;
        let w3: Point3<Real> = na::from_homogeneous(&w4);
        // Ray's origin is at eye, direction is at (point - eye)
        Ray::new(&self._eye, &(w3 - self._eye))
    }
}
