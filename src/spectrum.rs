use std::ops::Add;
use color::Color;


pub struct Spectrum {
    _color: Color,
}

impl Spectrum {
    #[inline] pub fn to_color(&self) -> Color { self._color }
}

impl Add for Spectrum {
    type Output = Spectrum;

    fn add(self, rhs: Spectrum) -> Spectrum {
        Spectrum {
            _color: self.to_color() + rhs.to_color()
        }
    }
}
