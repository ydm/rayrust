extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq };

use rayrust::primitive;
use rayrust::ray::{ self, Intersectable };


#[test]
fn sphere_intersection() {
    let sphere = primitive::Sphere::new(
        &na::Vector3::new(0.0, 0.0, -5.0), // center
        1.0                                // radius
    );

    let d = na::Vector3::<f32>::new(0.0, 0.0, -1.0);

    // Two intersections
    let ray2 = ray::Ray::<f32>::new(&na::Vector3::new(0.0, 0.0, 5.0), &d);
    let ts2 = sphere.intersections(&ray2);
    assert_eq!(ts2.len(), 2);
    assert_approx_eq!(ts2[0],  9.0);
    assert_approx_eq!(ts2[1], 11.0);
    assert_eq!(ts2[0], sphere.intersection(&ray2).unwrap());

    // One intersection
    let ray1 = ray::Ray::<f32>::new(&na::Vector3::new(1.0, 0.0, 5.0), &d);
    let ts1 = sphere.intersections(&ray1);
    assert_eq!(ts1.len(), 1);
    assert_approx_eq!(ts1[0], 10.0);
    assert_eq!(ts1[0], sphere.intersection(&ray1).unwrap());

    // Zero intersections
    let ray0 = ray::Ray::<f32>::new(&na::Vector3::new(1.1, 0.0, 5.0), &d);
    let ts0 = sphere.intersections(&ray0);
    assert_eq!(ts0.len(), 0);
    assert!(sphere.intersection(&ray0).is_none());
}
