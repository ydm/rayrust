use na;
use na::{ Matrix4, Point3, Point4, Vector3, Vector4 };

use camera::common;
use ray;
use types::{ Real };


pub struct OrthographicCamera {
    _raster_to_world: Matrix4<Real>,
}

impl OrthographicCamera {
    /// Create an orthographic camera
    ///
    /// * `width` - Image width
    /// * `height` - Image height
    /// * `plane` - Orthographic projection's clipping pane height
    /// * `eye` - Camera's location
    /// * `center` - Where the camera is looking at
    /// * `up` - Camera's up vector
    pub fn new(width: usize,
               height: usize,
               plane: Real,
               eye: &Point3<Real>,
               center: &Point3<Real>,
               up: &Vector3<Real>) -> OrthographicCamera {
        //
        let aspect = (width as Real) / (height as Real);
        OrthographicCamera {
            _raster_to_world: common::camera_to_world(&eye, &center, &up)
                * common::screen_to_ortho_camera(plane, aspect)
                * common::ndc_to_screen()
                * common::raster_to_ndc(width, height),
        }
    }
}


impl common::Camera for OrthographicCamera {
    fn generate_ray(&self, x: usize, y: usize) -> ray::Ray {
        let origin    = Point4 ::new(x as Real, y as Real,  0.0, 1.0);
        let direction = Vector4::new(0 as Real,       0.0, -1.0, 0.0);
        ray::Ray::new(&na::from_homogeneous(&(self._raster_to_world * origin)),
                      &na::from_homogeneous(&(self._raster_to_world * direction)))
    }
}

