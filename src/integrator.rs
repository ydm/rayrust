use camera::common::Camera;
use image::Image;
use scene::Scene;


pub trait Integrator {
    fn render(&self, scene: &Scene);
}

pub struct SamplerIntegrator {
    _camera: Box<Camera>,
    // TODO: Also has sampler
}

impl Integrator for SamplerIntegrator {
    fn render(&self, scene: &Scene) {
        let width = 800;
        let height = 600;

        // Film?
        let mut img = Image::new(width, height);

        for row in 0..img.height() {
            for col in 0..img.width() {
                // let camera_sample = sampler.get_camera_sample()
                // let ray = camera.generate(camera_sample)
                let ray = self._camera.generate_ray(col, row);

                // ...

                // intersect?

                // material?

                // add sample to film tile?

                // merge to film?
            }
        }
    }
}
