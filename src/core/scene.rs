use core::interaction::{ Intersectable, SurfaceInteraction };
use core::light::Light;
use core::ray::Ray;
use core::types::Real;


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
    #[inline] pub fn lights(&self) -> &Vec<Box<Light>> { &self._lights }
}

impl Intersectable for Scene {
    #[inline] fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self._aggregate.intersect(ray)
    }
}
