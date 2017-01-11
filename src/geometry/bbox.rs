use std::ops::Mul;
use na;
use na::{ Matrix4, Point3 };
use types::{ Real, RealMod };


const PMIN: Point3<Real> = Point3 { x: RealMod::INFINITY,
                                    y: RealMod::INFINITY,
                                    z: RealMod::INFINITY, };
const PMAX: Point3<Real> = Point3 { x: RealMod::NEG_INFINITY,
                                    y: RealMod::NEG_INFINITY,
                                    z: RealMod::NEG_INFINITY, };


pub struct BoundingBox {
    _pmin: Point3<Real>, 
    _pmax: Point3<Real>, 
}

impl BoundingBox {
    pub fn pmin(&self) -> Point3<Real> { self._pmin }
    pub fn pmax(&self) -> Point3<Real> { self._pmax }
}

impl Default for BoundingBox {
    fn default() -> BoundingBox {
        BoundingBox { _pmin: PMIN, _pmax: PMAX }
    }
}

impl Mul<BoundingBox> for Matrix4<Real> {
    type Output = BoundingBox;
    fn mul(self, rhs: BoundingBox) -> BoundingBox {
        let f = |p: &Point3<Real>| na::from_homogeneous(&(
            self * na::to_homogeneous(p)
        ));
        BoundingBox { _pmin: f(&rhs._pmin),
                      _pmax: f(&rhs._pmax) }
    }
}
