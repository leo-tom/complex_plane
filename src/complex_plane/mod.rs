/*
Copyright (C) <2018>  <Leo Reo Tomura>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
extern crate image;
extern crate num_complex;
extern crate num;
extern crate num_traits;


use self::num_complex::Complex;
use complex_func::ComplexNode;
use complex_func::complex_definition::ComplexDefinition;
use self::image::ImageBuffer;
use self::image::Rgba;
use std::u32;
use std::path::Path;
use std::iter::Iterator;
use complex_func::CalculationError;

#[allow(dead_code)]
pub struct ComplexPlane<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + PartialOrd> {
    from: Complex<T>,
    to: Complex<T>,
    buff: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}
#[allow(dead_code)]
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + PartialOrd>
    ComplexPlane<T> {
    pub fn new(z1: &Complex<T>, z2: &Complex<T>, w: u32, h: u32) -> ComplexPlane<T> {
        ComplexPlane {
            buff: ImageBuffer::new(w, h),
            from: Complex::new(
                if z1.re < z2.re {
                    z1.re.clone()
                } else {
                    z2.re.clone()
                },
                if z1.im < z2.im {
                    z1.im.clone()
                } else {
                    z2.im.clone()
                },
            ),
            to: Complex::new(
                if z1.re > z2.re {
                    z1.re.clone()
                } else {
                    z2.re.clone()
                },
                if z1.im > z2.im {
                    z1.im.clone()
                } else {
                    z2.im.clone()
                },
            ),
        }
    }
    pub fn width(&self) -> u32 {
        self.buff.width()
    }
    pub fn height(&self) -> u32 {
        self.buff.height()
    }
    fn x_zoom_factor(&self) -> f64 {
        let width = self.width() as f64;
        let range = (self.to.re.clone() - self.from.re.clone())
            .to_f64()
            .unwrap();
        width / range

    }
    fn y_zoom_factor(&self) -> f64 {
        let height = self.height() as f64;
        let range = (self.to.im.clone() - self.from.im.clone())
            .to_f64()
            .unwrap();
        height / range
    }
    fn get_coordinate(&self, z: &Complex<T>) -> (u32, u32) {
        let x = (z.re.clone() - self.from.re.clone()).to_f64().unwrap();
        let y = (z.im.clone() - self.from.im.clone()).to_f64().unwrap();
        let x_zoom = self.x_zoom_factor();
        let y_zoom = self.y_zoom_factor();

        let x = (x * x_zoom) as u32;
        let y = if self.height() >= (y * y_zoom) as u32 {
            self.height() - (y * y_zoom) as u32
        } else {
            0
        };
        (x, y)
    }
    pub fn get_range(&self) -> (Complex<T>, Complex<T>) {
        (self.from.clone(), self.to.clone())
    }
    pub fn put_dot(&mut self, p: &Complex<T>) {
        let rgb = 0x000000ff as u32;
        self.put_pixel(p, rgb);
    }
    pub fn put_dots(&mut self, v: &Vec<Complex<T>>) {
        for z in v {
            self.put_dot(z);
        }
    }
    pub fn put_pixel(&mut self, p: &Complex<T>, rgba: u32) {
        let color = Rgba {
            data: [
                (0xff & (rgba >> 24)) as u8,
                (0xff & (rgba >> 16)) as u8,
                (0xff & (rgba >> 8)) as u8,
                (0xff & rgba) as u8,
            ],
        };
        let (x, y) = self.get_coordinate(p);
        if x < self.buff.width() && y < self.buff.height() {
            //println!("x == {} y == {} {:?}", x, y, color);
            self.buff.put_pixel(x, y, color);
        }
    }
    pub fn put_pixels(&mut self, v: &Vec<Complex<T>>, rgba: u32) {
        for ref z in v {
            self.put_pixel(z, rgba);
        }
    }
    pub fn draw_pixels(&mut self, v: &Vec<(Complex<T>, u32)>) {
        for &(ref z, ref rgb) in v {
            self.put_pixel(z, *rgb);
        }
    }
    pub fn map_to(
        &self,
        mut plane: Self,
        n: ComplexNode<T>,
        mut def: ComplexDefinition<T>,
        vari: &str,
        rgba: u32,
    ) -> Result<Self, CalculationError> {
        let x_zoom = 1.0 / self.x_zoom_factor();
        let y_zoom = 1.0 / self.y_zoom_factor();
        let from = Complex::new(
            self.from.re.to_f64().unwrap(),
            self.from.im.to_f64().unwrap(),
        );
        def.define_numeric(
            vari,
            ComplexNode::fromc(Complex::new(
                T::from_f64(0.0 * x_zoom + from.re).unwrap(),
                T::from_f64(0.0 * y_zoom + from.im).unwrap(),
            )),
        );
        for x in 0..self.width() {
            for y in 0..self.height() {
                let real = (x as f64) * x_zoom + from.re;
                let imag = (y as f64) * y_zoom + from.im;
                let node = ComplexNode::fromc(Complex::new(
                    T::from_f64(real).unwrap(),
                    T::from_f64(imag).unwrap(),
                ));
                def.define_numeric(vari, node);
                let new = n.calculate(&def)?;
                plane.put_pixel(&new, rgba);
            }
        }
        Ok(plane)
    }
    pub fn map(
        &self,
        n: ComplexNode<T>,
        mut def: ComplexDefinition<T>,
        vari: &str,
        rgba: u32,
    ) -> Result<Self, CalculationError> {
        let mut vec = Vec::<Complex<T>>::new();
        let x_zoom = 1.0 / self.x_zoom_factor();
        let y_zoom = 1.0 / self.y_zoom_factor();
        let from = Complex::new(
            self.from.re.to_f64().unwrap(),
            self.from.im.to_f64().unwrap(),
        );
        def.define_numeric(
            vari,
            ComplexNode::fromc(Complex::new(
                T::from_f64(0.0 * x_zoom + from.re).unwrap(),
                T::from_f64(0.0 * y_zoom + from.im).unwrap(),
            )),
        );
        let mut min = n.calculate(&def)?;
        let mut max = min.clone();
        let buff = ImageBuffer::new(self.width(), self.height());
        for x in 0..self.width() {
            for y in 0..self.height() {
                let real = (x as f64) * x_zoom + from.re;
                let imag = (y as f64) * y_zoom + from.im;
                let node = ComplexNode::fromc(Complex::new(
                    T::from_f64(real).unwrap(),
                    T::from_f64(imag).unwrap(),
                ));
                def.define_numeric(vari, node);
                let new = n.calculate(&def)?;
                vec.push(new.clone());
                if new.re > max.re {
                    max.re = new.re.clone();
                } else if new.re < min.re {
                    min.re = new.re.clone();
                }
                if new.im > max.im {
                    max.im = new.im;
                } else if new.im < min.im {
                    min.im = new.im;
                }

            }
        }
        let mut plane = ComplexPlane {
            from: min.clone(),
            to: max.clone(),
            buff: buff,
        };
        plane.put_pixels(&vec, rgba);
        Ok(plane)
    }
    pub fn draw_fractal(&mut self, c: Complex<T>) {
        let c = Complex::new(c.re.to_f64().unwrap(), c.im.to_f64().unwrap());
        for x in (0..self.width()).collect::<Vec<u32>>() {
            for y in (0..self.height()).collect::<Vec<u32>>() {
                let x_zoom = (self.to.re.clone() - self.from.re.clone())
                    .to_f64()
                    .unwrap() / self.width() as f64;
                let y_zoom = (self.to.im.clone() - self.from.im.clone())
                    .to_f64()
                    .unwrap() / self.height() as f64;
                let mut z = Complex::new((x as f64) * x_zoom, (y as f64) * y_zoom);
                let mut val = 0 as u8;
                for n in 0..255 {
                    if z.norm() > 2.0 {
                        val = n;
                        break;
                    }
                    z = z * z + c;
                }
                let y = self.height() - y - 1;
                let color = Rgba { data: [val, 0, 0, val] };
                self.buff.put_pixel(x, y, color);
            }
        }
    }
    pub fn draw_axis(&mut self, rgba: u32) {
        let (o_x, o_y) = self.get_coordinate(&Complex::new(T::zero(), T::zero()));
        let color = Rgba {
            data: [
                (0xff & (rgba >> 24)) as u8,
                (0xff & (rgba >> 16)) as u8,
                (0xff & (rgba >> 8)) as u8,
                (0xff & rgba) as u8,
            ],
        };
        for x in 0..o_x {
            self.buff.put_pixel(x, o_y, color);
        }
        for x in o_x..(self.width()) {
            self.buff.put_pixel(x, o_y, color);
        }
        for y in 0..o_y {
            self.buff.put_pixel(o_x, y, color);
        }
        for y in o_y..(self.width()) {
            self.buff.put_pixel(o_x, y, color);
        }
    }
    pub fn save(&self, p: &Path) {
        self.buff.save(p).unwrap();
    }
}
