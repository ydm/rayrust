use na;
use na::{ Point3 };

use ray;
use types::{ Real };


// ------------------------
// Plane (TODO)
// ------------------------

pub struct Plane {
    // _normal: Vector3<Real>,
    // _d: Real,
}


// ------------------------
// Sphere
// ------------------------

pub struct Sphere {
    _center: Point3<Real>,
    _radius: Real,
}

impl Sphere {
    pub fn new(center: &Point3<Real>, radius: Real) -> Sphere {
        Sphere { _center: *center, _radius: radius }
    }

    pub fn center(&self) -> &Point3<Real> {
        &self._center
    }
}

impl ray::Intersectable for Sphere {
    fn intersections(&self, ray: &ray::Ray) -> Vec<Real> {
        let v = *ray.origin() - self._center;

        // Solve the quadratic equation
        //
        // a is 1 by definition
        let b = -na::dot(&v, ray.direction());
        let c = na::norm_squared(&v) - self._radius;
        let d = 4.0 * (b*b - c);

        match d {
            _ if d < 0.0 => vec![],   // zero intersections
                     0.0 => vec![b],  // one  intersection
                       _ => {         // two  intersections
                           let k = d.sqrt() / 2.0;
                           vec![b-k, b+k]
                       }
        }
    }
}
