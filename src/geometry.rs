use std::ops::Mul;
use na;
use na::{ Matrix4, Point3 };
use types::{ Real, RealMod };
use ray::{ Ray };


// ------------------------
// BoundingBox
// ------------------------

pub struct BoundingBox {
    _pmin: Point3<Real>, 
    _pmax: Point3<Real>, 
}

impl BoundingBox {
    pub fn pmin(&self) -> Point3<Real> { self._pmin }
    pub fn pmax(&self) -> Point3<Real> { self._pmax }
}


const PMIN: Point3<Real> = Point3 { x: RealMod::INFINITY,
                                    y: RealMod::INFINITY,
                                    z: RealMod::INFINITY, };
const PMAX: Point3<Real> = Point3 { x: RealMod::NEG_INFINITY,
                                    y: RealMod::NEG_INFINITY,
                                    z: RealMod::NEG_INFINITY, };

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


// ------------------------
// Shape
// ------------------------

trait Shape {
    
    /// Computes the surface area of a shape in object space.
    fn area(&self) -> Real { 0.0 }
    /// True if shape is intersectable, false otherwise.  intersect()
    /// should be called if and only if this method returns true.  The
    /// renderer assumes this method always returns a hard-coded
    /// constant value.
    fn can_intersect(&self) -> bool { true }
    // TODO: fn intersect(&self, r: Ray, &t_hit, &ray_epsilon, &differential_geometry)
    /// Predicate function that determines whether or not an inter-
    /// section occurs, without returning any details about the
    /// intersection itself.
    fn intersectp(&self, _: Ray) -> bool { true }
    /// Return a bounding box in the shape's object space.
    fn object_bound(&self) -> BoundingBox { BoundingBox::default() }
    /// Return the object-space to world-space transformation matrix.
    fn object_to_world(&self) -> Matrix4<Real> { na::one() }
    /// Return a bounding box in world space./// Return a bounding box
    /// in world space.
    fn world_bound(&self) -> BoundingBox {
        self.object_to_world() * self.object_bound()
    }
}
