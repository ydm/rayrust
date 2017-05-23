use na::{ Point2, Point3, Vector3 };
use core::spectrum::Spectrum;
use core::types::Real;


pub struct SurfaceInteraction {
    _p: Point3<Real>,
    _uv: Point2<Real>,
    _wo: Vector3<Real>,
    _time: Real,
    _shape: Box<Shape>,
}

impl SurfaceInteraction {
    // Getters
    #[inline] pub fn point(&self) -> Point3<Real> { self._p }
    #[inline] pub fn uv(&self) -> Point2<Real> { self._uv }
    #[inline] pub fn wo(&self) -> Vector3<Real> { self._wo }
    #[inline] pub fn time(&self) -> Real { self.__time }
    #[inline] pub fn shape(&self) -> Box<Shape> { self._shape }

    pub fn le(&self, w: &Vector3<Real>) -> Spectrum {
        
    }
}
