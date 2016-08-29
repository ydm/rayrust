extern crate nalgebra as na;

use super::types::{ Real };


// ------------------------
// Conversions
// ------------------------

pub fn m4_from_3v3(x: &na::Vector3<Real>,
                   y: &na::Vector3<Real>,
                   z: &na::Vector3<Real>)
//
                   -> na::Matrix4<Real> {
    na::Matrix4::new(
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn m4_from_4v3(x: &na::Vector3<Real>,
                   y: &na::Vector3<Real>,
                   z: &na::Vector3<Real>,
                   t: &na::Vector3<Real>)
//
                   -> na::Matrix4<Real> {
    na::Matrix4::new(
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        t.x, t.y, t.z, 1.0
    )
}

pub fn p3_from_p4(p: &na::Point4<Real>) -> na::Point3<Real> {
    na::Point3::new(p.x, p.y, p.z)
}

pub fn v3_from_p3(p: &na::Point3<Real>) -> na::Vector3<Real> {
    na::Vector3::new(p.x, p.y, p.z)
}

pub fn v3_from_v4(v: &na::Vector4<Real>) -> na::Vector3<Real> {
    na::Vector3::new(v.x, v.y, v.z)
}


// ------------------------
// Matrix helpers
// ------------------------

pub fn scale3f(x: Real, y: Real, z: Real) -> na::Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    na::Matrix4::new(
        x, o, o, o,
        o, y, o, o,
        o, o, z, o,
        o, o, o, l
    )
}

pub fn translate3f(x: Real, y: Real, z: Real) -> na::Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    na::Matrix4::new(
        l, o, o, o,
        o, l, o, o,
        o, o, l, o,
        x, y, z, l
    )
}
