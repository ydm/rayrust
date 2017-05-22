use na;
use na::{ Matrix4 };
use core::bbox::BoundingBox;
use core::ray::{ Intersectable };
use core::types::Real;


pub trait Shape: Intersectable {

    // Bounds
    // ------------------------

    /// Return a bounding box in the shape's object space.
    fn object_bound(&self) -> BoundingBox {
        BoundingBox::default()
    }

    /// Return a bounding box in world space./// Return a bounding box
    /// in world space.
    fn world_bound(&self) -> BoundingBox {
        self.world_from_object() * self.object_bound()
    }

    // Transformations
    // ------------------------

    /// Return the object-space to world-space transformation matrix.
    fn world_from_object(&self) -> Matrix4<Real> { na::one() }

    /// Return the world-space to object-space transformation matrix.
    fn object_from_world(&self) -> Matrix4<Real> { na::one() }

    // Other
    // ------------------------

    /// Computes the surface area of a shape in object space.
    fn area(&self) -> Real { 0.0 }
}
