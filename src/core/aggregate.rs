use core::primitive::Primitive;
use core::ray::{ Intersectable, Ray };
use core::types::Real;


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
    fn intersect(&self, ray: &Ray) -> Option<Real> {
        self._primitives.iter().fold(None, |memo, prim| {
            if let Some(x) = prim.intersect(ray) {
                if ray.is_inside(x) {
                    return if let Some(t) = memo {
                        Some(t.min(x))
                    } else {
                        Some(x)
                    }
                }
            }
            memo
        })
    }
}
