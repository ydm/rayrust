use na::{ Point4, Vector3, Matrix4 };
use types::{ Real };


// ------------------------
// nalgebra isn't a beauty
// ------------------------

/// Negate just the x, y, z components of a point, leaving its w value
/// unchanged.
pub fn negp(p: &Point4<Real>) -> Point4<Real> {
    let mut ans = -*p;
    ans[3] = p[3];
    ans
}


// ------------------------
// Matrix constructors
// ------------------------

pub fn m3v(x: &Vector3<Real>,
           y: &Vector3<Real>,
           z: &Vector3<Real>)
//
           -> Matrix4<Real> {
    Matrix4::new(
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn m4v(x: &Vector3<Real>,
           y: &Vector3<Real>,
           z: &Vector3<Real>,
           t: &Vector3<Real>)
//
           -> Matrix4<Real> {
    Matrix4::new(
        x.x, x.y, x.z, 0.0,
        y.x, y.y, y.z, 0.0,
        z.x, z.y, z.z, 0.0,
        t.x, t.y, t.z, 1.0
    )
}


// ------------------------
// Matrix helpers
// ------------------------

pub fn scale3f(x: Real, y: Real, z: Real) -> Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    Matrix4::new(
        x, o, o, o,
        o, y, o, o,
        o, o, z, o,
        o, o, o, l
    )
}

pub fn translate3f(x: Real, y: Real, z: Real) -> Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    Matrix4::new(
        l, o, o, o,
        o, l, o, o,
        o, o, l, o,
        x, y, z, l
    )
}
