use na::{ Point3 };

use types::{ Real, RealMod };


pub struct BoundingBox {
    _pmin: Point3<Real>, 
    _pmax: Point3<Real>, 
}

impl BoundingBox {
    // TODO: new_with_points(p, q)
    // TODO: new_with_point(p)
    
    pub fn pmin(&self) -> Point3<Real> { self._pmin }
    pub fn pmax(&self) -> Point3<Real> { self._pmax }
}


const MIN: Point3<Real> = Point3 { x: RealMod::INFINITY,
                                   y: RealMod::INFINITY,
                                   z: RealMod::INFINITY, };
const MAX: Point3<Real> = Point3 { x: RealMod::NEG_INFINITY,
                                   y: RealMod::NEG_INFINITY,
                                   z: RealMod::NEG_INFINITY, };

impl Default for BoundingBox {
    fn default() -> BoundingBox {
        BoundingBox { _pmin: MIN, _pmax: MAX }
    }
}
