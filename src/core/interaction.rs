use na::{ Point2, Point3, Vector3 };
use core::ray::Ray;
use core::shape::Shape;
use core::spectrum::Spectrum;
use core::types::Real;


// ------------------------
// SurfaceInteraction
// ------------------------

pub struct SurfaceInteraction {
    // All interactions have a point `p' and `time' associated with
    // them.
    _d: Real,
    // _p: Point3<Real>,
    _time: Real,

    // Some data is specific for surface interactions:
    _n: Vector3<Real>,
    _uv: Point2<Real>,
    // _shape: Box<Shape>,
}

impl SurfaceInteraction {
    pub fn new(d: Real, time: Real, n: &Vector3<Real>, uv: &Point2<Real>)
               -> SurfaceInteraction {
        SurfaceInteraction {
            _d: d,
            _time: time,
            _n: *n,
            _uv: *uv,
        }
    }
    // Getters
    // #[inline] pub fn point(&self) -> Point3<Real> { self._p }
    #[inline] pub fn d(&self) -> Real { self._d }
    #[inline] pub fn time(&self) -> Real { self._time }
    #[inline] pub fn normal(&self) -> Vector3<Real> { self._n }
    #[inline] pub fn uv(&self) -> Point2<Real> { self._uv }
    // #[inline] pub fn shape(&self) -> Box<Shape> { self._shape }

    #[inline] pub fn compute_scattering_fs(&self) {
        
    }

    pub fn le(&self, w: &Vector3<Real>) -> Spectrum {
        Spectrum::default()
    }
}


// ------------------------
// Intersectable
// ------------------------

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}
