extern crate nalgebra as na;

use self::na::Norm;
use super::ray;
use super::types::{ Real };


pub struct Sphere<T> {
    _center: na::Point4<T>,
    _radius: T,
}

impl<T: Copy> Sphere<T> {
    pub fn new(center: &na::Point4<T>, radius: T) -> Sphere<T> {
        Sphere { _center: *center, _radius: radius }
    }
}

impl ray::Intersectable<Real> for Sphere<Real> {
    fn intersections(&self, ray: &ray::Ray<Real>) -> Vec<Real> {
        let v: na::Vector4<Real> = *ray.origin() - self._center;

        // Solve the quadratic equation
        //
        // a is 1 by definition
        let b = na::dot(&v, ray.direction());
        let c = v.norm_squared() - self._radius;
        let d = 4.0 * (b*b - c);

        match d {
            _ if d < 0.0 => vec![],    // zero
                     0.0 => vec![-b],  // one
                       _ => {          // two
                           let k = d.sqrt() / 2.0;
                           vec![-b-k, -b+k]
                       }
        }
    }
}
