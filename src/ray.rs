extern crate nalgebra as na;

use std::clone::Clone;

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
    _origin: na::Point4<T>,
    _direction: na::Vector4<T>,
}

impl<T: Copy> Ray<T> {
    pub fn new(o: &na::Point4<T>, d: &na::Vector4<T>) -> Ray<T> {
        Ray {
            _origin: *o,
            _direction: *d,
        }
    }

    pub fn origin(&self) -> &na::Point4<T> {
        &self._origin
    }

    pub fn direction(&self) ->&na::Vector4<T> {
        &self._direction
    }
}
