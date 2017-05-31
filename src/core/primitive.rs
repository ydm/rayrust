use core::interaction::{ Intersectable, SurfaceInteraction };
use core::ray::Ray;
use core::shape::Shape;
use core::types::Real;


pub struct Primitive {
    _shape: Box<Shape>,
    // TODO: _material: Material;
}

impl Primitive {
    #[inline] pub fn new(shape: Box<Shape>) -> Primitive {
        Primitive {
            _shape: shape,
        }
    }

    // TODO: compute_scattering_functions
}

impl Intersectable for Primitive {
    #[inline] fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self._shape.intersect(ray)
    }
}
