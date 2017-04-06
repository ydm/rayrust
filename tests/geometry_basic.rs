extern crate rayrust;

#[macro_use]
extern crate approx;
use approx::ApproxEq;

extern crate nalgebra as na;
use na::{ Point3, Vector3 };

use rayrust::geometry::basic;
use rayrust::geometry::ray;
use rayrust::geometry::shape::Shape;
use rayrust::geometry::basic::Sphere;
    

#[test]
fn sphere_intersection_0() {
    let sphere = Sphere::new(&Point3::new(0.0, 0.0, -5.0), 1.0);
    let d = Vector3::new(0.0, 0.0, -1.0);

    // Zero intersections
    let ray0 = ray::Ray::new(&Point3::new(1.1, 0.0, 5.0), &d);
    let ts0 = sphere.intersect(&ray0);
    assert!(ts0.is_none());
}

#[test]
fn sphere_intersection_1() {
    let sphere = Sphere::new(&Point3::new(0.0, 0.0, -5.0), 1.0);
    let d = Vector3::new(0.0, 0.0, -1.0);

    // One intersection
    let ray1 = ray::Ray::new(&Point3::new(1.0, 0.0, 5.0), &d);
    let ts1 = sphere.intersections(&ray1);
    assert_eq!(ts1.len(), 1);
    assert_relative_eq!(ts1[0], 10.0);
    assert_eq!(ts1[0], sphere.intersection(&ray1).unwrap());
}

#[test]
fn sphere_intersection_2() {
    let sphere = Sphere::new(&Point3::new(0.0, 0.0, -5.0), 1.0);
    let d = Vector3::new(0.0, 0.0, -1.0);

    // Two intersections
    let ray2 = ray::Ray::new(&Point3::new(0.0, 0.0, 5.0), &d);
    let ts2 = sphere.intersections(&ray2);
    assert_eq!(ts2.len(), 2);
    assert_relative_eq!(ts2[0],  9.0);
    assert_relative_eq!(ts2[1], 11.0);
    assert_eq!(ts2[0], sphere.intersection(&ray2).unwrap());
}

#[test]
fn sphere_intersection_3() {
    let sphere = Sphere::new(&Point3::new(0.0, 0.0, -5.0), 2.0);
    let d = Vector3::new(0.0, 0.0, -1.0);

    // Two intersections, greater radius
    let ray2 = ray::Ray::new(&Point3::new(0.0, 0.0, 5.0), &d);
    let ts2 = sphere.intersections(&ray2);
    assert_eq!(ts2.len(), 2);
    assert_relative_eq!(ts2[0],  8.0);
    assert_relative_eq!(ts2[1], 12.0);
    assert_eq!(ts2[0], sphere.intersection(&ray2).unwrap());
}
