use core::color::Color;
use std::default::Default;
use std::ops::Add;


pub struct Spectrum {
    _color: Color,
}

impl Spectrum {
    pub fn new(color: &Color) -> Spectrum {
        Spectrum { _color: *color, }
    }
    #[inline] pub fn to_color(&self) -> Color { self._color }
}

impl Default for Spectrum {
    fn default() -> Self {
        Spectrum { _color: Color::default(), }
    }
}

impl Add for Spectrum {
    type Output = Spectrum;

    fn add(self, rhs: Spectrum) -> Self::Output {
        Spectrum {
            _color: self.to_color() + rhs.to_color()
        }
    }
}
