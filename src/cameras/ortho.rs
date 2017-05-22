use na::{ Matrix4, Point3, Point4, Vector3 };

use core::camera;
use core::ray::Ray;
use core::types::Real;


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
            _raster_to_world: camera::world_from_camera(&eye, &center, &up) *
                camera::ortho_camera_from_screen(plane, aspect)             *
                camera::screen_from_ndc()                                   *
                camera::ndc_from_raster(width, height),
        }
    }
}

impl camera::Camera for OrthographicCamera {
    fn generate_ray(&self, x: usize, y: usize) -> Ray {
        let o = Point4::new(x as Real, y as Real, 0.0, 1.0);

        let wo =  self._raster_to_world * o;       // world origin
        let wd = -self._raster_to_world.column(2); // world dir

        let wo3 = Point3::from_homogeneous(wo.coords).unwrap();
        let wd3 = Vector3::from_homogeneous(wd).unwrap();

        Ray::new(&wo3, &wd3)
    }
}
