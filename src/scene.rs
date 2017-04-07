use geometry::ray::{ Intersectable, Ray };
use types::Real;


pub struct Scene {
    _aggregate: Box<Intersectable>,
}

impl Scene {
    #[inline] pub fn new(a: Box<Intersectable>) -> Scene {
        Scene {
            _aggregate: a,
        }
    }
}

impl Intersectable for Scene {
    #[inline] fn intersect(&self, ray: &Ray) -> Option<Real> {
        self._aggregate.intersect(ray)
    }
}
