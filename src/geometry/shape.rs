use na;
use na::{ Matrix4 };
use geometry::bbox::{ BoundingBox };
use geometry::ray::{ Hit, Ray };
use types::{ Real };


trait Shape {
    // Intersections
    // ------------------------

    /// True if shape is intersectable, false otherwise.  intersect()
    /// should be called if and only if this method returns true.  The
    /// renderer assumes this method always returns a hard-coded
    /// constant value.
    fn can_intersect(&self) -> bool { true }

    // TODO: fn intersect(&self, r: Ray, &t_hit, &ray_epsilon, &differential_geometry)

    fn intersect(&self, _: Ray) -> Hit { Hit::default() }

    /// Predicate function that determines whether or not an inter-
    /// section occurs, without returning any details about the
    /// intersection itself.
    fn intersectp(&self, _: Ray) -> bool { true }

    // Bounds
    // ------------------------

    /// Return a bounding box in the shape's object space.
    fn object_bound(&self) -> BoundingBox {
        BoundingBox::default()
    }

    /// Return a bounding box in world space./// Return a bounding box
    /// in world space.
    fn world_bound(&self) -> BoundingBox {
        self.object_to_world() * self.object_bound()
    }

    // Transformations
    // ------------------------

    /// Return the object-space to world-space transformation matrix.
    fn object_to_world(&self) -> Matrix4<Real> { na::one() }

    fn world_to_object(&self) -> Matrix4<Real> { na::one() }

    // Other
    // ------------------------

    /// Computes the surface area of a shape in object space.
    fn area(&self) -> Real { 0.0 }
}
