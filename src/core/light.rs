use core::ray::Ray;
use core::spectrum::Spectrum;


pub trait Light {
    fn le(&self, ray: &Ray) -> Spectrum;
    // TODO
}
