use na::{ Point3, Vector3 };
use core::types::Real;
// use spectrum::Spectrum;


pub struct SurfaceInteraction {
    /// Hit point
    _p: Point3<Real>,

    /// Normal
    _n: Vector3<Real>,

    /// Negative ray direction
    _wo: Vector3<Real>,

    /// Ray hit time.  `wo` stands for the notation of outgoing
    /// direction when computing lighting at point.
    _time: Real,

    #[inline] fn point(&self) -> &Point3<Real> { self._p }
    #[inline] fn normal(&self) -> &Vector3<Real> { self._n }
    #[inline] fn wo(&self) -> &Vector3<Real> { self._wo }
    #[inline] fn time(&self) -> Real { self._time }

    fn le(&self, v: &Vector3) -> Box<Spectrum> {
    }
}
