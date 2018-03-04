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
        let new_def = ComplexNode::<T>::parse(def);
        let new_var_def = ComplexNode::<T>::parse(name);
        
        match new_var_def {
            Some(x) => {
            	let right = x.right.clone();
            	match right {
            		Some(arg) => {
            			//WTF
            			println!("aa {}",x.to_string());
            			self.map.insert(
                            x.to_string(),
                            ComplexValue::Func(
                                *arg,
                                *new_def.expect(&format!("Failed to parse content of {}.", name)),
                            ),
                        );
            		}
            		_ => {
            			if x.is_const() {
            				self.map.insert(
	                            String::from(name),
	                            ComplexValue::Value(*x),
	                        );
            			}else{
            				self.map.insert(
	                            String::from(name),
	                            ComplexValue::Value(*x),
	                        );
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
    pub fn get(&self, name: &str) -> Complex<T> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.const_calculate(),
                    &ComplexValue::Func(_, _) => {
                        panic!("Tried to call fucntion {} as variable", name)
                    }
                }
            }
            None => panic!("No definition for {}", name),
        }
    }
    pub fn is_variable(&self,name:&str) -> bool {
    	match self.map.get(name) {
    		Some(x) => match x {
    			&ComplexValue::Value(_) => true,
    			_ => false,
    		}
    		_ => false,
    	}
    }
    pub fn is_function(&self,name:&str) -> bool{
    	!self.is_variable(name)
    }
    pub fn call(&self, name: &str, arguments: &ComplexNode<T>) -> Complex<T> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.calculate(self),
                    &ComplexValue::Func(ref def_arg, _) => {
                        let mut def = self.clone();
                        let vecter = arguments.get_vec();
                        let mut index = 0;
                        for name in def_arg.get_vec() {
                            def.define_numeric(&name.to_string(), vecter[index].clone());
                            index += 1;
                        }
                        self.get_with_definition(name, &def)
                    }
                }
            }
            _ => panic!(format!("{} is not defined",name)),
        }
    }
    pub fn get_with_definition(&self, name: &str, definition: &ComplexDefinition<T>) -> Complex<T> {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.calculate(definition),
                    &ComplexValue::Func(_, ref f) => f.calculate(definition),

                }
            }
            _ => panic!("No definition for {}", name),
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
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Default
    for ComplexDefinition<T> {
    fn default() -> Self {
        let _1 = (
            String::from("i"),
            ComplexValue::Value(ComplexNode::fromc(&Complex::new(T::zero(),T::one()))),
        );
        let _2 = (
            String::from("e"),
            ComplexValue::Value(ComplexNode::fromc(&Complex::new(T::from_f64(consts::E).unwrap(),T::zero()))),
        );
        let _3 = (
            String::from("PI"),
            ComplexValue::Value(ComplexNode::fromc(&Complex::new(T::from_f64(consts::PI).unwrap(),T::zero()))),
        );
        let vec: Vec<(String, ComplexValue<T>)> = vec![_1, _2,_3];
        let mut def = ComplexDefinition { map: HashMap::<String, ComplexValue<T>>::from_iter(vec) };
        def.define("exp(x)", "e^x");
        def
    }
}
