extern crate nalgebra as na;

use super::types::{ Real };


// ------------------------
// nalgebra isn't a beauty
// ------------------------




// ------------------------
// Matrix constructors
// ------------------------

pub fn mfrom3v(x: &na::Vector4<Real>,
               y: &na::Vector4<Real>,
               z: &na::Vector4<Real>)
//
               -> na::Matrix4<Real> {
    na::Matrix4::new(
        x.x, x.y, x.z, x.w,
        y.x, y.y, y.z, y.w,
        z.x, z.y, z.z, z.w,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn mfrom4v(x: &na::Vector4<Real>,
               y: &na::Vector4<Real>,
               z: &na::Vector4<Real>,
               t: &na::Vector4<Real>)
//
               -> na::Matrix4<Real> {
    na::Matrix4::new(
        x.x, x.y, x.z, x.w,
        y.x, y.y, y.z, y.w,
        z.x, z.y, z.z, z.w,
        t.x, t.y, t.z, t.w
    )
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
