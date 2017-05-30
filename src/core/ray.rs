use na;
use na::{ Point3, Vector3 };
use core::types::{ Real, RealMod };

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
    #[inline] pub fn new(o: &Point3<Real>, d: &Vector3<Real>) -> Ray {
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

    /// Determine whether t lies between tmin and tmax.
    #[inline]
    pub fn is_inside(&self, t: Real) -> bool {
        self._tmin <= t && t <= self._tmax
    }
}
