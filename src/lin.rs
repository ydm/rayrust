extern crate nalgebra as na;

use super::types::{ Real };


// ------------------------
// nalgebra isn't a beauty
// ------------------------

pub fn cross(p: &na::Vector4<Real>, q: &na::Vector4<Real>)
             -> na::Vector4<Real> {
    v((p.y * q.z) - (p.z * q.y),
      (p.z * q.x) - (p.x * q.z),
      (p.x * q.y) - (p.y * q.x))
}

pub fn p(x: Real, y: Real, z: Real) -> na::Point4<Real> {
    na::Point4::<Real>::new(x, y, z, 1.0)
}

pub fn v(x: Real, y: Real, z: Real) -> na::Vector4<Real> {
    na::Vector4::<Real>::new(x, y, z, 0.0)
}

/// Negate just the x, y, z components of a point, leaving its w value
/// unchanged.
pub fn negp(p: &na::Point4<Real>) -> na::Point4<Real> {
    let mut ans = -*p;
    ans[3] = p[3];
    ans
}


// ------------------------
// Matrix constructors
// ------------------------

pub fn m3v(x: &na::Vector4<Real>,
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

pub fn m4v(x: &na::Vector4<Real>,
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
