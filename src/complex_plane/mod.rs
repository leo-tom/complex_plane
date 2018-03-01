extern crate image;
extern crate num_complex;
extern crate num;
extern crate num_traits;


use self::num_complex::Complex;
use self::image::ImageBuffer;
use std::u32;
use std::path::Path;
use std::iter::Iterator;


#[allow(dead_code)]
pub struct Plane<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + PartialOrd> {
    from: Complex<T>,
    to: Complex<T>,
    buff: image::RgbImage,
}
#[allow(dead_code)]
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + PartialOrd>
    Plane<T> {
    pub fn new(z1: &Complex<T>, z2: &Complex<T>, w: u32, h: u32) -> Plane<T> {
        Plane {
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
                    z2.im.clone()
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
    pub fn put_dot(&mut self, p: &Complex<T>) {
        let rgb = 0xffffff as u32;
        self.put_pixel(p, rgb);
    }
    pub fn put_dots(&mut self, v: &Vec<Complex<T>>) {
        for z in v {
            self.put_dot(z);
        }
    }
    pub fn put_pixel(&mut self, p: &Complex<T>, rgb: u32) {
        let color = image::Rgb {
            data: [
                (0xff & (rgb >> 16)) as u8,
                (0xff & (rgb >> 8)) as u8,
                (0xff & rgb) as u8,
            ],
        };
        let x = (p.re.clone() - self.from.re.clone()).to_f64().unwrap();
        let y = (p.im.clone() - self.from.im.clone()).to_f64().unwrap();
        let x_zoom = self.buff.width() as f64 /
            (self.to.re.clone() - self.from.re.clone())
                .to_f64()
                .unwrap();
        let y_zoom = self.buff.height() as f64 /
            (self.to.im.clone() - self.from.im.clone())
                .to_f64()
                .unwrap();

        let x = (x * x_zoom) as u32;
        let y = self.height() - (y * y_zoom) as u32;

        if x < self.buff.width() && y < self.buff.height() {
            //println!("x == {} y == {} {:?}", x, y, color);
            self.buff.put_pixel(x, y, color);
        }
    }
    pub fn put_pixels(&mut self, v: &Vec<(Complex<T>, u32)>) {
        for &(ref z, ref rgb) in v {
            self.put_pixel(z, *rgb);
        }
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
                let color = image::Rgb { data: [val, 0, 0] };
                self.buff.put_pixel(x, y, color);
            }
        }
    }
    pub fn save(&self, p: &Path) {
        self.buff.save(p).unwrap();
    }
}
