use std::ops::Mul;
use na;
use na::{ Matrix4, Point3 };
use types::{ Real, RealMod };


const PMIN: Point3<Real> = Point3::new(RealMod::INFINITY,
                                       RealMod::INFINITY,
                                       RealMod::INFINITY);
const PMAX: Point3<Real> = Point3::new(RealMod::NEG_INFINITY,
                                       RealMod::NEG_INFINITY,
                                       RealMod::NEG_INFINITY);


pub struct BoundingBox {
    _pmin: Point3<Real>, 
    _pmax: Point3<Real>, 
}

impl BoundingBox {
    #[inline]
    pub fn pmin(&self) -> Point3<Real> { self._pmin }
    #[inline]
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
        let f = |p: &Point3<Real>| Point3::from_homogeneous(
            self * p.to_homogeneous()).unwrap();
        BoundingBox { _pmin: f(&rhs._pmin),
                      _pmax: f(&rhs._pmax) }
    }
}
