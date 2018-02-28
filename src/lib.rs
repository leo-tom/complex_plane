extern crate num_complex;
pub mod complex_plane;
pub mod complex_func;
#[cfg(test)]
mod tests {
    use complex_plane::Plane;
    use complex_func::ComplexNode;
    use num_complex::Complex;
    use std::path::Path;
    use std::thread;
    #[test]
    fn it_works() {

        /*
        let handler1 = thread::spawn(|| {
            let z1 = Complex::new(0.0, 0.0);
            let z2 = Complex::new(0.4, 0.4);
            let c = Complex::new(-0.4051234123, 0.60124312);
            let mut f = Plane::new(&z1, &z2, 100, 100);
            f.draw_fractal(c);
            let path = Path::new("out.png");
            f.save(&path);
        });
        */

        //handler1.join().unwrap();
        let parsed = ComplexNode::parse("2+x*42/2").expect("FUCK");
        println!("{}", parsed);
    }

}
