use scene::Scene;


pub trait Integrator {
    fn render(scene: &Scene);
}

pub trait SamplerIntegrator: Integrator {
    // has camera
    // has sampler
}
