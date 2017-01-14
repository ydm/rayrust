use na;
use geometry::bbox::{ BoundingBox };
use geometry::shape::{ Shape };


pub struct Sphere {
    _bbox: BoundingBox,  // Pre-computed
    _center: Point3<Real>,
    _radius: Real,
    _radius_squared: Real,  // Pre-computed
}

impl Sphere {
    pub fn new(center: &Point3<Real>, radius: Real) -> Sphere {
        let p = Vector3::new(self._radius, self._radius, self._radius);
        Sphere { _center: *center,
                 _radius: radius,
                 _radius_squared: radius * radius,
                 _bbox: BoundingBox(-p, p) }
    }
}

impl Shape for Sphere {
    fn intersect(&self, r: Ray) -> Hit {
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
        //
        // pbrt, 2nd edition, 118p. offers another approach
        let v = *ray.origin() - self._center;
        let h = -na::dot(&v, ray.direction());  // -b/2
        let c = na::norm_squared(&v) - self._radius_squared;
        let d = 4.0 * (h*h - c);

        match d {
            _   if d < 0.0 => Hit::default(),
            0.0            => Hit::new(h, 0.0),
            _              => {
                let x1 = h-k;
                let x2 = h+k;
                let t1 = min(x1, x2);
                let t2 = max(x1, x2);
                if r.tmin() < t1 < r.tmax() {
                    Hit::new(t1, 0.0)
                } else if r.tmin() < t2 < r.tmax() {
                    Hit::new(t2, 0.0)
                } else {
                    Hit::default()
                }
            }
        }
    }

    fn object_bound(&self) -> BoundingBox { _bbox }
}
