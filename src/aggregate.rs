use geometry::ray::{ Intersectable, Ray };
use primitive::Primitive;
use types::Real;


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
            let intersection = prim.intersect(ray);
            match intersection {
                Some(x) => match memo {
                    Some(t) if t < x => memo,
                    _                => Some(x)
                },
                None    => memo
            }
        })
    }
}
