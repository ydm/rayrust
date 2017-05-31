use core::camera::Camera;
use core::color::Color;
use core::image::Image;
use core::interaction::Intersectable;
use core::ray::Ray;
use core::scene::Scene;
use core::spectrum::Spectrum;


// ------------------------
// Integrator
// ------------------------

pub trait Integrator {
    // TODO: Should return a Film?
    fn render(&self, scene: &Scene, camera: &Camera) -> Box<Image>;
}


// ------------------------
// SamplerIntegrator
// ------------------------

pub struct SamplerIntegrator {
    // TODO: Aggregates a sampler!
}

impl SamplerIntegrator {
    pub fn new() -> SamplerIntegrator {
        SamplerIntegrator {}
    }

    fn li(ray: &Ray, scene: &Scene) -> Spectrum {
        match scene.intersect(ray) {
            Some(x) => {
                // let y = x / 8.0;
                let y = 1.0;
                Spectrum::new(&Color::new(y, y, y, y))
            },

            // No intersection, return background radiance.
            None => scene.lights().iter().fold(
                Spectrum::default(),
                |memo, ref x| memo + x.le(ray)
            )
        }
    }
}

impl Integrator for SamplerIntegrator {
    fn render(&self, scene: &Scene, camera: &Camera) -> Box<Image> {
        let width = 800;
        let height = 600;

        // multithreaded for each tile
        //   for each sample, ray & differentials
        //     let camera_sample = sampler.get_camera_sample()
        //     let ray & diff = camera.gen(camera_sample)

        let mut img = Box::new(Image::new(width, height));
        for row in 0..img.height() {
            for col in 0..img.width() {

                let ray = camera.generate_ray(col, row);
                let res = Self::li(&ray, scene);
                img.set(col, row, &res.to_color());
            }
        }

        img
    }
}
