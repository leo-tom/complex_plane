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
use complex_plane::Plane;
use complex_func::ComplexNode;
use num_complex::Complex;
use std::collections::hash_map::Keys;
use std::fmt;
use std::iter::FromIterator;
use std::f64::consts;

#[derive(Debug, Clone)]
enum ComplexValue<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    // vecter of arguments , definition
    Func(ComplexNode<T>, ComplexNode<T>),
    Value(ComplexNode<T>),
}

#[derive(Debug)]
pub struct ComplexDefinition<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    map: HashMap<String, ComplexValue<T>>,
}

impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
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
        let new_var_def = ComplexNode::<T>::parse(name);
        let new_def = ComplexNode::<T>::parse(def);
        match new_var_def {
            Some(x) => {
                let right = x.right.clone();
                match right {
                    Some(arg) => {
                        let fname = x.get_name();
                        self.map.insert(
                            fname,
                            ComplexValue::Func(
                                *arg,
                                *new_def.expect(&format!("Failed to parse content of {}.", name)),
                            ),
                        );
                    }
                    _ => {
                        if x.is_const() {
                            self.map.insert(String::from(name), ComplexValue::Value(*x));
                        } else {
                            self.map.insert(String::from(name), ComplexValue::Value(*x));
                        }
                    }
                }
            }
            _ => panic!("Failed to parse {}", name),
        }
    }
    pub fn remove(&mut self, name: &str) {
        self.map.remove(name);
    }
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
    pub fn get(&self, name: &str) -> ComplexNode<T> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.clone(),
                    &ComplexValue::Func(_, ref f) => f.clone(),
                }
            }
            None => panic!("No definition for {}", name),
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
    pub fn call(&self, name: &str, arguments: &ComplexNode<T>) -> Complex<T> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.calculate(self),
                    &ComplexValue::Func(ref def_arg, ref f) => {
                        if name == "real" {
                            return Complex::new(arguments.calculate(self).re, T::zero());
                        }
                        let mut def = self.clone();
                        let vecter = arguments.get_vec();
                        let mut index = 0;
                        for name in def_arg.get_vec() {
                            def.define_numeric(
                                &name.to_string(),
                                ComplexNode::fromc(vecter[index].calculate(self)),
                            );

                            index += 1;
                        }
                        f.calculate(&def)
                    }
                }
            }
            _ => panic!(format!("{} is not defined", name)),
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
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> fmt::Display
    for ComplexDefinition<T> {
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
        def.define("real(x)", "real(x)");
        def.define("exp(x)", "e^x");
        def.define("cos(x)", "(1/2)*(exp(i*x)+exp(-i*x))");
        def.define("sin(x)", "(1/2i)*(exp(i*x)-exp(-i*x))");
        def.define("tan(x)", "sin(x)/cos(x)");
        def.define("sqrt(x)", "x^(1/2)");
        def.define("abs(x)", "sqrt(norm(x))");
        def.define("imaginary(x)", "x - real(x)");
        def.define("norm(x)", "x*(real(x)-imaginary(x))");

        def
    }
}
