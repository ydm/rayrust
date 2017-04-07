use geometry::ray::{ Intersectable, Ray };
use geometry::shape::Shape;
use types::Real;


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
}

impl Intersectable for Primitive {
    #[inline] fn intersect(&self, ray: &Ray) -> Option<Real> {
        self._shape.intersect(ray)
    }
}
