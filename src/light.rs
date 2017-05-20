use geometry::ray::Ray;
use spectrum::Spectrum;


pub trait Light {
    fn le(&self, ray: &Ray) -> Spectrum;
    // TODO
}
