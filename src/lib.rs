
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

    use std::time::{Duration, SystemTime};




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
        handler1.join().unwrap();
        */

        let formula = "((1+2)*(2+32))-421/4+(4+2)/2";
        let start_parse = SystemTime::now();
        let parsed = ComplexNode::<f64>::parse(formula).expect("FUCK");
        match start_parse.elapsed() {
            Ok(x) => {
                println!(
                    "Parsing took: {}ns,{}s",
                    x.subsec_nanos(),
                    ((x.subsec_nanos() as f64) / 1000000000.0)
                )
            }
            _ => panic!("FUCK"), 
        }
        //println!("{}", parsed);
        let calculated = parsed.const_calculate();
        let start_calculation = SystemTime::now();
        match start_calculation.elapsed() {
            Ok(x) => println!("Calculating took : {}ns", x.subsec_nanos()),
            _ => panic!("FUCK"),
        }
        println!("{} == {}", formula, calculated);
    }

}
