use std::io;
use color;


/// Types that can be converted to a [u8; 4] RGBA.
trait ToRGBA255 {
    fn rgba255(&self) -> [u8; 4];
}

impl ToRGBA255 for color::Color {
    fn rgba255(&self) -> [u8; 4] {
        let f = |x: f32| (x * 255.0) as u8;        
        [ f(self.channels()[0]),
          f(self.channels()[1]),
          f(self.channels()[2]),
          f(self.channels()[3]) ]
    }
}

pub struct Image {
    _data: Vec<color::Color>,
    _width: usize,
    _height: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        Image {
            _data: vec![color::Color::default(); w * h],
            _width: w,
            _height: h
        }
    }

    pub fn width(&self) -> usize { self._width }
    pub fn height(&self) -> usize { self._height }

    pub fn writeppm<T>(&self, out: &mut T) -> io::Result<()>
        where T: io::Write {
        //
        try!(write!(out, "P6 {} {} 255 ", self._width, self._height));
        for color in &self._data {
            let channels = &color.rgba255()[0..3];
            try!(out.write_all(&channels));
        }
        Ok(())
    }

    pub fn set(&mut self, x: usize, y: usize, c: &color::Color) {
        self._data[x + (y * self._width)] = *c;
    }
}
