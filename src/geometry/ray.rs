use na;
use na::{ Point3, Vector3 };
use types::{ Real, RealMod };


pub trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Option<Real> {
        self.intersections(ray).first().cloned()
    }

    fn intersections(&self, ray: &Ray) -> Vec<Real>;
}


// ------------------------
// Hit
// ------------------------

pub struct Hit {
    _t: Real,  // Hit parameter
    _e: Real,  // Epsilon
    // TODO: DifferentialGeometry, ShadingGeometry
}

impl Hit {
    pub fn new(t: Real, e: Real) -> Hit {
        Hit { _t: t, _e: e}
    }

    pub fn t(&self) -> Real { self._t }
    pub fn e(&self) -> Real { self._e }
}

impl Default for Hit {
    fn default() -> Hit {
        Hit { _t: -1.0, _e: 0.0 }
    }
}


// ------------------------
// Ray
// ------------------------

#[derive(Debug)]
pub struct Ray {
    _origin: Point3<Real>,
    _direction: Vector3<Real>,
    _tmin: Real,
    _tmax: Real,
}

impl Ray {
    pub fn new(o: &Point3<Real>, d: &Vector3<Real>) -> Ray {
        Ray {
            _origin: *o,
            _direction: na::normalize(d),
            _tmin: 0.0,
            _tmax: RealMod::INFINITY,
        }
    }

    pub fn origin(&self) -> &Point3<Real> {
        &self._origin
    }

    pub fn direction(&self) ->&Vector3<Real> {
        &self._direction
    }
}
