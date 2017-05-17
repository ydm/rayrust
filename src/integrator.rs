use camera::common::Camera;
use color::Color;
use geometry::ray::{ Intersectable, Ray };
use image::Image;
use scene::Scene;


// ------------------------
// Integrator
// ------------------------

pub trait Integrator {
    fn render(&self, scene: &Scene, camera: &Camera) -> Box<Image>;
}


// ------------------------
// SamplerIntegrator
// ------------------------

pub struct SamplerIntegrator {
    // _camera: Box<Camera>,
    // TODO: Also has sampler
}

impl SamplerIntegrator {
    pub fn new() -> SamplerIntegrator {
        SamplerIntegrator {}
    }

    fn li(ray: &Ray, scene: &Scene) -> Color {
        // let o = scene.intersect(ray);
        match scene.intersect(ray) {
            Some(x) => { let y = x / 8.0; Color::new(y, y, y, y) },
            None => Color::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

impl Integrator for SamplerIntegrator {
    fn render(&self, scene: &Scene, camera: &Camera) -> Box<Image> {
        let width = 800;
        let height = 600;

        // for each tile
        //   for each sample, ray & differentials
        //     let camera_sample = sampler.get_camera_sample()
        //     let ray & diff = camera.gen(camera_sample)

        let mut img = Box::new(Image::new(width, height));
        for row in 0..img.height() {
            for col in 0..img.width() {

                let ray = camera.generate_ray(col, row);
                let res = Self::li(&ray, scene);
                img.set(col, row, &res);
            }
        }

        img
    }
}
