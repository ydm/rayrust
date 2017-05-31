use na;
use na::{ Point2, Point3, Vector3 };

use core::interaction::{ Intersectable, SurfaceInteraction };
use core::ray::Ray;
use core::shape::Shape;
use core::types::Real;


pub struct Sphere {
    _center: Point3<Real>,
    _radius: Real,
    _radius_squared: Real,  // Cached for faster intersections
}

impl Sphere {
    pub fn new(center: &Point3<Real>, radius: Real) -> Sphere {
        Sphere { _center: *center,
                 _radius: radius,
                 _radius_squared: radius * radius, }
    }

    pub fn center(&self) -> &Point3<Real> {
        &self._center
    }

    fn hit(&self, ray: &Ray) -> Option<Real> {
        // Ray: w + d*t
        //      * w is origin point
        //      * d is direction
        //      * t is direction parameter
        // Sphere: ||o-p|| = r
        //      * o is sphere center
        //      * p is any sphere point
        //      * r is radius
        //
        // Sphere points satisfy the following equation:
        // ||v + d*t|| = r , where v = w-o
        //
        // Thus ||d||^2 * t  + 2*v*d*t + ||v||^2 - r^2 = 0
        //
        // Now solve the quadratic equation for
        // a = ||d||^2, which is 1
        // b = 2 * dot(v, d)
        // c = ||v||^2 - r^2
        let v = *ray.origin() - self._center;
        let h = -na::dot(&v, ray.direction());  // -b/2
        let c = na::norm_squared(&v) - self._radius_squared;
        let d = 4.0 * (h*h - c);

        if d < 0.0 {
            // No intersections
            None
        } else if d > 0.0 {
            // Two intersections
            let k = d.sqrt() / 2.0;
            if      ray.is_inside(h - k) { Some(h - k) }
            else if ray.is_inside(h + k) { Some(h + k) }
            else    { None }            
        } else {
            // One intersection
            if ray.is_inside(h) { Some(h) } else { None }
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        match self.hit(ray) {
            Some(x) => {
                let p: Point3<Real> = ray.origin() + (ray.direction() * x);
                let n: Vector3<Real> = na::normalize(&(p - self._center));
                let uv: Point2<Real> = Point2::new(0.0, 0.0);
                Some(SurfaceInteraction::new(x, 0.0, &n, &uv))
            },
            None => None
        }
    }
}

impl Shape for Sphere {
    // TODO
}
