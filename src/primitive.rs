extern crate nalgebra as na;

use self::na::Norm;
use super::ray;


pub struct Sphere<T> {
    _center: na::Vector3<T>,
    _radius: T,
}

impl<T: Copy> Sphere<T> {
    pub fn new(center: &na::Vector3<T>, radius: T) -> Sphere<T> {
        Sphere { _center: *center, _radius: radius }
    }
}

impl ray::Intersectable<f32> for Sphere<f32> {
    fn intersections(&self, ray: &ray::Ray<f32>) -> Vec<f32> {
        let v: na::Vector3<f32> = *ray.origin() - self._center;

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
