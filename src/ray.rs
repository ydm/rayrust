use std::clone::Clone;
use na::{ Point3, Vector3 };


pub trait Intersectable<T: Clone> {
    fn intersection(&self, ray: &Ray<T>) -> Option<T> {
        match self.intersections(ray).first() {
            Some(x) => Some((*x).clone()),
            None => None,
        }
    }

    fn intersections(&self, ray: &Ray<T>) -> Vec<T>;
}

#[derive(Debug)]
pub struct Ray<T> {
    _origin: Point3<T>,
    _direction: Vector3<T>,
}

impl<T: Copy> Ray<T> {
    pub fn new(o: &Point3<T>, d: &Vector3<T>) -> Ray<T> {
        Ray {
            _origin: *o,
            _direction: *d,
        }
    }

    pub fn origin(&self) -> &Point3<T> {
        &self._origin
    }

    pub fn direction(&self) ->&Vector3<T> {
        &self._direction
    }
}
