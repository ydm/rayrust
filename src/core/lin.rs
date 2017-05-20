use na::{ Point4, Vector3, Matrix4 };
use types::{ Real };


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
           z: &Vector3<Real>) -> Matrix4<Real> {
    Matrix4::new(
        x.x, y.x, z.x, 0.0,
        x.y, y.y, z.y, 0.0,
        x.z, y.z, z.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn m4v(x: &Vector3<Real>,
           y: &Vector3<Real>,
           z: &Vector3<Real>,
           p: &Vector3<Real>) -> Matrix4<Real> {
    //
    Matrix4::new(
        x.x, y.x, z.x, p.x,
        x.y, y.y, z.y, p.y,
        x.z, y.z, z.z, p.z,
        0.0, 0.0, 0.0, 1.0
    )
}


// ------------------------
// Matrix helpers
// ------------------------

pub fn scale2f(x: Real, y: Real) -> Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    Matrix4::new(
        x, o, o, o,
        o, y, o, o,
        o, o, l, o,
        o, o, o, l
    )
}

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

pub fn translate2f(x: Real, y: Real) -> Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    //  x, y, z, p
    Matrix4::new(
        l, o, o, x,
        o, l, o, y,
        o, o, l, o,
        o, o, o, l
    )
}

pub fn translate3f(x: Real, y: Real, z: Real) -> Matrix4<Real> {
    let l = 1.0;
    let o = 0.0;
    //  x, y, z, p
    Matrix4::new(
        l, o, o, x,
        o, l, o, y,
        o, o, l, z,
        o, o, o, l
    )
}
