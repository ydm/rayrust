use core::ray::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Real>;
}
