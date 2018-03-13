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
extern crate regex;
extern crate num_traits;
extern crate num;

use std::collections::HashMap;
use complex_func::ComplexNode;
use num_complex::Complex;
use std::fmt;
use std::iter::FromIterator;
use std::f64::consts;
use complex_func::CalculationError;
use std::sync::Arc;


enum ComplexValue<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + 'static> {
    // (vecter of name of arguments , definition of function as formula)
    Func(ComplexNode<T>, ComplexNode<T>),
    NaitiveFunc(
        Arc<
            Fn(ComplexNode<T>, ComplexDefinition<T>) -> Result<Complex<T>, CalculationError>
                + 'static,
        >
    ),
    Value(ComplexNode<T>),
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Clone
    for ComplexValue<T> {
    fn clone(&self) -> Self {
        match self {
            &ComplexValue::NaitiveFunc(ref f) => ComplexValue::NaitiveFunc(f.clone()),
            &ComplexValue::Func(ref x, ref y) => ComplexValue::Func(x.clone(), y.clone()),
            &ComplexValue::Value(ref x) => ComplexValue::Value(x.clone()),
        }
    }
}
pub struct ComplexDefinition<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + 'static> {
    map: HashMap<String, ComplexValue<T>>,
}

impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + 'static>
    ComplexDefinition<T> {
    pub fn new() -> ComplexDefinition<T> {
        ComplexDefinition { map: HashMap::new() }
    }
    pub fn define_numeric(&mut self, name: &str, value: ComplexNode<T>) {
        self.map.insert(
            String::from(name),
            ComplexValue::Value(value),
        );
    }
    pub fn define_naitive_function(
        &mut self,
        name: &str,
        f: Arc<Fn(ComplexNode<T>, ComplexDefinition<T>) -> Result<Complex<T>, CalculationError>>,
    ) {
        self.map.insert(
            name.to_owned(),
            ComplexValue::NaitiveFunc(f),
        );
    }
    pub fn define_function(&mut self, name: &str, var_def: ComplexNode<T>, def: ComplexNode<T>) {
        self.map.insert(
            String::from(name),
            ComplexValue::Func(var_def, def),
        );
    }
    pub fn define_from_definition(&mut self, definitions: ComplexDefinition<T>) {
        self.map.extend(definitions.map.into_iter());
    }
    pub fn define(&mut self, name: &str, def: &str) {
        let parsed_name =
            ComplexNode::<T>::parse(name).expect(&format!("Failed to parse {}.", name));
        let parsed_def = ComplexNode::<T>::parse(def).expect(&format!(
            "Failed to parse content of {} : {}",
            name,
            def
        ));
        let right = parsed_name.right.clone();
        let fname = parsed_name.get_name();
        match right {
            Some(arg) => {
                self.map.insert(
                    fname,
                    ComplexValue::Func(*arg, *parsed_def),
                );
            }
            _ => {
                self.map.insert(fname, ComplexValue::Value(*parsed_def));
            }
        }
    }
    pub fn remove(&mut self, name: &str) {
        self.map.remove(name);
    }
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
    pub fn get(&self, name: &str) -> Result<ComplexNode<T>, CalculationError> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => Ok(v.clone()),
                    &ComplexValue::Func(_, ref f) => Ok(f.clone()),
                    &ComplexValue::NaitiveFunc(_) => {
                        Err(CalculationError::Unknown(
                            format!(
                                "You can not get {}. Because it is built-in function or value.",
                                name
                            ).to_owned(),
                        ))
                    }

                }
            }
            None => Err(CalculationError::ValueNotDefined(name.to_owned())),
        }
    }
    pub fn is_variable(&self, name: &str) -> bool {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(_) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn is_function(&self, name: &str) -> bool {
        !self.is_variable(name)
    }
    pub fn call(
        &self,
        name: &str,
        arguments: &ComplexNode<T>,
    ) -> Result<Complex<T>, CalculationError> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.calculate(self),
                    &ComplexValue::Func(ref def_arg, ref f) => {
                        let mut def = self.clone();
                        let vecter = arguments.get_vec();
                        let mut index = 0;
                        for name in def_arg.get_vec() {
                            def.define_numeric(
                                &name.to_string(),
                                ComplexNode::fromc(vecter[index].calculate(self)?),
                            );

                            index += 1;
                        }
                        f.calculate(&def)
                    }
                    &ComplexValue::NaitiveFunc(ref f) => f(arguments.clone(), self.clone()),
                }
            }
            _ => Err(CalculationError::ValueNotDefined(name.to_owned())),
        }
    }
    pub fn get_keys(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = Vec::new();
        for k in self.map.keys() {
            vec.push(k);
        }
        vec
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Clone
    for ComplexDefinition<T> {
    fn clone(&self) -> Self {
        let mut map = HashMap::<String, ComplexValue<T>>::new();
        for (key, val) in self.map.iter() {
            map.insert(key.clone(), val.clone());
        }
        ComplexDefinition { map: map }
    }
}
impl<
    T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone + 'static,
> fmt::Display for ComplexDefinition<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = String::new();
        for kee in self.get_keys() {
            result.push_str(kee);
            result.push_str(":\n");
            match self.map.get(kee) {
                Some(x) => {
                    match x {
                        &ComplexValue::Func(_, ref f) => result.push_str(&format!("{}", f)),
                        &ComplexValue::Value(ref v) => result.push_str(&format!("{}", v)),
                        &ComplexValue::NaitiveFunc(_) => result.push_str("###NAITIVE CODE###"),
                    }
                }
                _ => (),
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Default
    for ComplexDefinition<T> {
    fn default() -> Self {
        let _1 = (
            String::from("i"),
            ComplexValue::Value(ComplexNode::fromc(Complex::new(T::zero(), T::one()))),
        );
        let _2 = (
            String::from("e"),
            ComplexValue::Value(ComplexNode::fromc(
                Complex::new(T::from_f64(consts::E).unwrap(), T::zero()),
            )),
        );
        let _3 = (
            String::from("PI"),
            ComplexValue::Value(ComplexNode::fromc(
                Complex::new(T::from_f64(consts::PI).unwrap(), T::zero()),
            )),
        );
        let vec: Vec<(String, ComplexValue<T>)> = vec![_1, _2, _3];
        let mut def = ComplexDefinition { map: HashMap::<String, ComplexValue<T>>::from_iter(vec) };

        def.define_naitive_function("real", Arc::from(real));
        def.define_naitive_function("imag", Arc::from(imag));
        def.define_naitive_function("acos", Arc::from(acos));
        def.define_naitive_function("asin", Arc::from(asin));
        def.define_naitive_function("atan", Arc::from(atan));
        def.define_naitive_function("arg", Arc::from(arg));
        def.define_naitive_function("ln", Arc::from(ln));
        def.define_naitive_function("log", Arc::from(log));
        def.define("exp(x)", "e^x");
        def.define("cos(x)", "(1/2)*(exp(i*x)+exp(-i*x))");
        def.define("sin(x)", "(1/2i)*(exp(i*x)-exp(-i*x))");
        def.define("tan(x)", "sin(x)/cos(x)");
        def.define("sqrt(x)", "x^(1/2)");
        def.define("abs(x)", "sqrt(norm(x))");
        def.define("norm(x)", "x*(real(x)-imaginary(x))");
        def.define("to_radians(x)", "(x/180)*PI");
        def.define("to_degrees(x)", "x*(180/PI)");

        return def;
        /*DEFINITION OF BUILT_IN_FUNCTION*/
        fn real<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            Ok(Complex::from(arg.calculate(&def)?.re))
        }
        fn imag<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            Ok(Complex::new(T::zero(), arg.calculate(&def)?.im))
        }
        fn acos<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            let arg = arg.calculate(&def)?;
            let float = Complex::new(arg.re.to_f64().unwrap(), arg.im.to_f64().unwrap());
            let retval = float.acos();
            let retval = Complex::new(
                T::from_f64(retval.re).unwrap(),
                T::from_f64(retval.im).unwrap(),
            );
            Ok(retval)
        }
        fn asin<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            let arg = arg.calculate(&def)?;
            let float = Complex::new(arg.re.to_f64().unwrap(), arg.im.to_f64().unwrap());
            let retval = float.asin();
            let retval = Complex::new(
                T::from_f64(retval.re).unwrap(),
                T::from_f64(retval.im).unwrap(),
            );
            Ok(retval)
        }
        fn atan<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            let arg = arg.calculate(&def)?;
            let float = Complex::new(arg.re.to_f64().unwrap(), arg.im.to_f64().unwrap());
            let retval = float.atan();
            let retval = Complex::new(
                T::from_f64(retval.re).unwrap(),
                T::from_f64(retval.im).unwrap(),
            );
            Ok(retval)
        }
        fn arg<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            let arg = arg.calculate(&def)?;
            let float = Complex::new(arg.re.to_f64().unwrap(), arg.im.to_f64().unwrap());
            let retval = float.arg();
            let retval = Complex::new(T::from_f64(retval).unwrap(), T::zero());
            Ok(retval)
        }
        fn ln<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            let arg = arg.calculate(&def)?;
            let float = Complex::new(arg.re.to_f64().unwrap(), arg.im.to_f64().unwrap());
            let retval = float.ln();
            let retval = Complex::new(
                T::from_f64(retval.re).unwrap(),
                T::from_f64(retval.im).unwrap(),
            );
            Ok(retval)
        }
        fn log<
            T: num::traits::Num
                + num_traits::ToPrimitive
                + num_traits::FromPrimitive
                + Clone
                + 'static,
        >(
            arg: ComplexNode<T>,
            def: ComplexDefinition<T>,
        ) -> Result<Complex<T>, CalculationError> {
            /*log(base,self)*/
            if !arg.is_vec() {
                return Err(CalculationError::Unknown(
                    "log function requires two variables. log(x,y) returns base-x logarithm of y."
                        .to_owned(),
                ));
            }
            let as_vec = arg.get_vec();
            if as_vec.len() < 2 {
                return Err(CalculationError::Unknown(
                    "log function requires two variables. log(x,y) returns base-x logarithm of y."
                        .to_owned(),
                ));
            }
            let x = as_vec[1].calculate(&def)?;
            let base = as_vec[0].calculate(&def)?;
            let x = Complex::new(x.re.to_f64().unwrap(), x.im.to_f64().unwrap());
            let base = Complex::new(base.re.to_f64().unwrap(), base.im.to_f64().unwrap());
            let retval = x.log(base.re);
            let retval = Complex::new(
                T::from_f64(retval.re).unwrap(),
                T::from_f64(retval.im).unwrap(),
            );
            Ok(retval)
        }
    }
}
