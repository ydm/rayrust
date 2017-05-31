use core::interaction::{ Intersectable, SurfaceInteraction };
use core::primitive::Primitive;
use core::ray::Ray;


pub struct LinearAggregate {
    _primitives: Vec<Primitive>,
}

impl LinearAggregate {
    #[inline]
    pub fn new(prims: Vec<Primitive>) -> LinearAggregate {
        LinearAggregate { _primitives: prims }
    }
}

impl Intersectable for LinearAggregate {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self._primitives.iter().fold(None, |memo, prim| {
            match (memo, prim.intersect(ray)) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => if a.d() < b.d() { Some(a) } else { Some(b) }
            }
        })
    }
}
