#![allow(dead_code)]

use std::io;

// ------------------------
// Color
// ------------------------

#[derive(Clone, Debug)]
struct Color {
    channels: [u8; 4],
}


impl Color {
    fn new() -> Color { Color { channels: [0; 4] } }
    fn channels(&self) -> [u8; 4] { self.channels }
    fn red(&self) -> u8 { self.channels[0] }
    fn green(&self) -> u8 { self.channels[1] }
    fn blue(&self) -> u8 { self.channels[2] }
    fn alpha(&self) -> u8 { self.channels[3] }
}


// ------------------------
// Image
// ------------------------

pub struct Image {
    data: Vec<Color>,
    width: u16,
    height: u16
}

impl Image {
    pub fn new(w: u16, h: u16) -> Image {
        let n: usize = (w as usize) * (h as usize);
        Image {
            data: vec![Color::new(); n],
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

    // pub fn set(&mut self, x: u16, y: u16, c: &Color) {
    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        let index = x * (self.width as usize) + y;
        self.data[x * self.width + y] = c;
    }
}
