use color::Color;


pub trait Spectrum {
    fn to_color(&self) -> Color;
}
