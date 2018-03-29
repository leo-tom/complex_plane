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
    use complex_plane::ComplexPlane;
    use complex_func::ComplexNode;
    use num_complex::Complex;
    use std::path::Path;
    use std::thread;
    use complex_func::complex_definition::ComplexDefinition;
    use std::f64::consts::PI;

    use std::error::Error;
    use std::time::SystemTime;


    #[test]
    fn plane_test() {
        let z1 = Complex::new(0.0, 0.0);
        let z2 = Complex::new(0.4, 0.4);
        let c = Complex::new(-0.4051234123, 0.60124312);
        let mut f = ComplexPlane::new(&z1, &z2, 500, 500);
        f.draw_fractal(c);
        let path = Path::new("out.png");
        f.save(&path);
    }

    //#[test]
    fn func_test() {
        //let formula = "(3+2i)*(3-2i)";
        let formula = "1+2-3*4/5^6";
        let start_parse_def = SystemTime::now();
        let def = ComplexDefinition::default();
        match start_parse_def.elapsed() {
            Ok(x) => {
                println!(
                    "ComplexDefinition::default() took: {}ns,{}s",
                    x.subsec_nanos(),
                    ((x.subsec_nanos() as f64) / 1000000000.0)
                )
            }
            _ => panic!("WHAT"),
        }
        let start_parse = SystemTime::now();
        let parsed = ComplexNode::<f64>::parse(formula).expect("FUCK");
        println!("{}", parsed);
        match start_parse.elapsed() {
            Ok(x) => {
                println!(
                    "Parsing took: {}ns,{}s",
                    x.subsec_nanos(),
                    ((x.subsec_nanos() as f64) / 1000000000.0)
                )
            }
            _ => panic!("WHAT"),
        }
        let start_calculation = SystemTime::now();
        let calculated = match parsed.calculate(&def) {
            Ok(v) => v,
            Err(e) => panic!("Error : {}", e),
        };

        match start_calculation.elapsed() {
            Ok(x) => {
                println!(
                    "Calculating took : {}ns,{}s",
                    x.subsec_nanos(),
                    ((x.subsec_nanos() as f64) / 1000000000.0)
                )
            }
            _ => panic!("WHAT"),
        }
        println!("{} == {}", formula, calculated);
        let start_drawing = SystemTime::now();
        let from = ComplexPlane::new(
            &Complex::new(0.0, 0.0),
            &Complex::new(2.0 * PI, 0.0),
            800,
            800,
        );
        let to = ComplexPlane::new(&Complex::new(-1.0, -1.0), &Complex::new(1.0, 1.0), 400, 400);
        let mut mapped = match from.map_to(
            to,
            *ComplexNode::<f64>::parse("exp(x*i)").unwrap(),
            def,
            "x",
            0x000000ff,
        ) {
            Ok(v) => v,
            Err(e) => panic!("{} : {}", e.description(), e),
        };
        match start_drawing.elapsed() {
            Ok(x) => {
                println!(
                    "Drawing took : {}ns,{}s",
                    x.subsec_nanos(),
                    ((x.subsec_nanos() as f64) / 1000000000.0)
                )
            }
            _ => panic!("WHAT"),
        }
        let path = Path::new("out.png");
        mapped.draw_axis(0x4286f4ff);
        mapped.save(path);
    }
}
