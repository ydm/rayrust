use std::io::{ Result, Write};
use core::color::Color;


/// Types that can be converted to a [u8; 4] RGBA.
trait ToRGBA255 {
    fn rgba255(&self) -> [u8; 4];
}

impl ToRGBA255 for Color {
    fn rgba255(&self) -> [u8; 4] {
        let f = |x: f32| (x * 255.0) as u8;
        [ f(self.channels()[0]),
          f(self.channels()[1]),
          f(self.channels()[2]),
          f(self.channels()[3]) ]
    }
}


// ------------------------
// Image
// ------------------------

pub struct Image {
    _data: Vec<Color>,
    _width: usize,
    _height: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        Image {
            _data: vec![Color::default(); w * h],
            _width: w,
            _height: h
        }
    }

    #[inline]
    pub fn width(&self) -> usize { self._width }

    #[inline]
    pub fn height(&self) -> usize { self._height }

    pub fn writeppm<T>(&self, out: &mut T) -> Result<()>
        where T: Write {
        //
        try!(write!(out, "P6 {} {} 255 ", self._width, self._height));
        for color in &self._data {
            let channels = &color.rgba255()[0..3];
            try!(out.write_all(&channels));
        }
        Ok(())
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        self._data[x + (y * self._width)] = *c;
    }
}


// ------------------------
// Spectrum
// ------------------------

