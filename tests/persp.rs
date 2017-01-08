extern crate rayrust;

#[macro_use(assert_approx_eq)]
extern crate nalgebra as na;

use na::{ ApproxEq, Point3, Point4, Vector3, Vector4 };

use rayrust::camera::persp;
use rayrust::types::{ Real };
// use rayrust::types::{ RealConsts };


#[test]
fn camera_to_world_0() {
    // let r = (90 as Real).to_radians();
    // let m = persp::camera_to_world(r, 1.0, 0.1, 100.0);

    // let v1 = Vector4::new(0.0, 0.0, -1.0, 0.0);
    // assert_approx_eq!(v1 * m, Vector4::new(0.0, 0.0, -1.0, 0.0));

    // let v2 = Vector4::new(0.0, 0.0, -1.0, 0.0);
    // assert_approx_eq!(v2 * m, Vector4::new(0.0, 0.0, -1.0, 0.0));
}

// #[test]
// fn fuck() {
//     let cam = persp::PerspectiveCamera::new(800, 800);
//     let gen = cam.generate_ray(400, 400);
//     assert_approx_eq!(*gen.origin(), Point3::new(2.0, -1.0, 5.0));
//     assert_approx_eq!(*gen.direction(), Vector3::new(0.0, 0.0, -1.0));
// }
