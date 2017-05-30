use na::{ Point2, Point3, Vector3 };
use core::spectrum::Spectrum;
use core::types::Real;


pub struct Shading {
    // TODO: normal, ray differentials, other?
}

pub struct Interaction {
    // All interactions have a point `p' and `time' associated with
    // them.
    _p: Point3<Real>,
    _time: Real,

    // Some data is specific for surface interactions:
    _n: Vector3<Real>,
    _uv: Point2<Real>,
    _shape: Box<Shape>,
}

impl Interaction {
    // Getters
    #[inline] pub fn point(&self) -> Point3<Real> { self._p }
    #[inline] pub fn time(&self) -> Real { self.__time }
    // Surface interaction getters
    #[inline] pub fn normal(&self) -> Vector3<Real> { self._n }
    #[inline] pub fn uv(&self) -> Point2<Real> { self._uv }
    #[inline] pub fn shape(&self) -> Box<Shape> { self._shape }

    pub fn le(&self, w: &Vector3<Real>) -> Spectrum {
        
    }
}
