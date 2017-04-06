use na;
use na::{ Point3, Vector3 };
use types::{ Real, RealMod };


pub trait Intersectable {
    // Intersections
    // ------------------------

    // /// True if shape is intersectable, false otherwise.  intersect()
    // /// should be called if and only if this method returns true.  The
    // /// renderer assumes this method always returns a hard-coded
    // /// constant value.
    // fn can_intersect(&self) -> bool { true }

    fn intersect(&self, ray: &Ray) -> Option<Real>;

    /// Predicate function that determines whether or not an
    /// intersection occurs, without returning any details about the
    /// intersection itself.
    fn intersectp(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}


// pub trait Intersectable {
//     fn intersect(&self, ray: &Ray) -> Option<Real>;

//     /// Predicate function that determines whether or not an
//     /// intersection occurs, without returning any details about the
//     /// intersection itself.
//     fn intersectp(&self, ray: &Ray) -> bool {
//         self.intersect(ray).is_some()
//     }
// }


// ------------------------
// Hit
// ------------------------

pub struct Hit {
    _t: Real,  // Hit parameter
    _e: Real,  // Epsilon
    // TODO: DifferentialGeometry, ShadingGeometry
}

impl Hit {
    pub fn new(t: Real, e: Real) -> Hit {
        Hit { _t: t, _e: e}
    }

    pub fn t(&self) -> Real { self._t }
    pub fn e(&self) -> Real { self._e }
}

impl Default for Hit {
    fn default() -> Hit {
        Hit { _t: -1.0, _e: 0.0 }
    }
}


// ------------------------
// Ray
// ------------------------

#[derive(Debug)]
pub struct Ray {
    _origin: Point3<Real>,
    _direction: Vector3<Real>,
    _tmin: Real,
    _tmax: Real,
}

impl Ray {
    pub fn new(o: &Point3<Real>, d: &Vector3<Real>) -> Ray {
        Ray {
            _origin: *o,
            _direction: na::normalize(d),
            _tmin: 0.0,
            _tmax: RealMod::INFINITY,
        }
    }

    #[inline] pub fn origin   (&self) -> &Point3<Real>  { &self._origin }
    #[inline] pub fn direction(&self) -> &Vector3<Real> { &self._direction }
    #[inline] pub fn tmin     (&self) -> Real           { self._tmin }
    #[inline] pub fn tmax     (&self) -> Real           { self._tmax }

    #[inline]
    pub fn inside(&self, t: Real) -> bool {
        self._tmin <= t && t <= self._tmax
    }
}
