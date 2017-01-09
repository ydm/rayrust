use na;
use na::{ Point3, Vector3 };
use types::{ Real };


pub trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Option<Real> {
        self.intersections(ray).first().cloned()
    }

    fn intersections(&self, ray: &Ray) -> Vec<Real>;
}

#[derive(Debug)]
pub struct Ray {
    _origin: Point3<Real>,
    _direction: Vector3<Real>,
}

impl Ray {
    pub fn new(o: &Point3<Real>, d: &Vector3<Real>) -> Ray {
        Ray {
            _origin: *o,
            _direction: na::normalize(d),
        }
    }

    pub fn origin(&self) -> &Point3<Real> {
        &self._origin
    }

    pub fn direction(&self) ->&Vector3<Real> {
        &self._direction
    }
}
