#![allow(dead_code)]

use std::default::Default;
use std::io;

// ------------------------
// Pixel
// ------------------------

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    channels: [u8; 4],
}


impl Pixel {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { channels: [r, g, b, a] }
    }
    fn channels(&self) -> [u8; 4] { self.channels }
    fn red(&self) -> u8 { self.channels[0] }
    fn green(&self) -> u8 { self.channels[1] }
    fn blue(&self) -> u8 { self.channels[2] }
    fn alpha(&self) -> u8 { self.channels[3] }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel { channels: [0u8; 4] }
    }
}


// ------------------------
// Image
// ------------------------

pub struct Image {
    data: Vec<Pixel>,
    width: usize,
    height: usize
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let n: usize = w * h;
        Image {
            data: vec![Pixel::default(); n],
            width: w,
            height: h
        }
    }

    pub fn write<T>(&self, out: &mut T) -> io::Result<()>
        where T: io::Write {
        //
        try!(write!(out, "P6 {} {} 255 ", self.width, self.height));
        for color in &self.data {
            try!(out.write_all(&color.channels()));
        }
        Ok(())
    }

    pub fn set(&mut self, x: usize, y: usize, c: &Pixel) {
        self.data[(x * (self.width as usize)) + y] = *c;
    }
}
