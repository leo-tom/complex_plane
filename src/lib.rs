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
    use complex_func::complex_definition::ComplexDefinition;

    use std::time::{Duration, SystemTime};




    //#[test]
    fn bench_test() {

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
    #[test]
    fn it_works() {
        let def = ComplexDefinition::default();
        let formula = "exp(2)";
        println!("{:?}", def);
        println!(
            "{} == {}",
            formula,
            ComplexNode::<f64>::parse(formula)
                .expect("FUCK")
                .calculate(&def)
        );
    }
}
