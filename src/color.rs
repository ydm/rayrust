use std::default::Default;


#[derive(Clone, Copy, Debug)]
pub struct Color {
    _channels: [f32; 4],
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { _channels: [r, g, b, a] }
    }
    pub fn channels(&self) -> &[f32; 4] { &self._channels }
    pub fn red(&self) -> f32 { self._channels[0] }
    pub fn green(&self) -> f32 { self._channels[1] }
    pub fn blue(&self) -> f32 { self._channels[2] }
    pub fn alpha(&self) -> f32 { self._channels[3] }
}

impl Default for Color {
    fn default() -> Self {
        Color { _channels: [0.0; 4] }
    }
}
