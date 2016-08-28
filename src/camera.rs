extern crate nalgebra as na;


struct Camera {
    // na::Matrix4
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
        }
    }
}

pub fn raster_to_ndc(w: u16, h: u16, p: &na::Vector3<u16>) -> na::Vector3<f32> {
    let x = (p.x as f32) / (w as f32);
    let y = (p.y as f32) / (h as f32);
    na::Vector3::new(x, y, 0.0)
}

// pub fn ndc_to_
