use geometry::ray::{ Intersectable, Ray };
use light::Light;
use types::Real;


pub struct Scene {
    _aggregate: Box<Intersectable>,
    _lights: Vec<Box<Light>>
}

impl Scene {
    #[inline] pub fn new(a: Box<Intersectable>) -> Scene {
        Scene {
            _aggregate: a,
            _lights: vec![]
        }
    }
    // #[inline] pub fn lights(&self) -> Vec<Light> {
    // }
}

impl Intersectable for Scene {
    #[inline] fn intersect(&self, ray: &Ray) -> Option<Real> {
        self._aggregate.intersect(ray)
    }
}
