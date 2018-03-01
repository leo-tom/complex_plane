extern crate num_complex;
extern crate regex;
extern crate num_traits;
extern crate num;

use std::collections::HashMap;
use complex_plane::Plane;
use complex_func::ComplexNode;
use num_complex::Complex;
use std::collections::hash_map::Keys;

#[derive(Debug)]
enum ComplexValue<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    Func(ComplexNode<T>),
    Value(T),
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
    pub fn define_numeric(&mut self, name: &str, value: T) {
        self.map.insert(
            String::from(name),
            ComplexValue::Value(value),
        );
    }
    pub fn define_function(&mut self, name: &str, definition: ComplexNode<T>) {
        self.map.insert(
            String::from(name),
            ComplexValue::Func(definition),
        );
    }
    pub fn define_from_definition(&mut self, definitions: ComplexDefinition<T>) {
        self.map.extend(definitions.map.into_iter());
    }
    pub fn define(&mut self, name: &str, definition: &str) {
        let new_def = ComplexNode::<T>::parse(definition);
        match new_def {
            Some(x) => {
                if x.is_const() {
                    self.map.insert(
                        String::from(name),
                        ComplexValue::Value(x.const_calculate()),
                    );
                } else {
                    self.define_function(name, *x);
                }
            }
            _ => panic!(format!("Failed to parse \"{}\".", definition)),
        }
    }
    pub fn remove(&mut self, name: &str) {
        self.map.remove(name);
    }
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
    pub fn get(&self, name: &str) -> T {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.clone(),
                    &ComplexValue::Func(ref f) => f.calculate(self),
                }
            }
            None => panic!("No definition for {}", name),
        }
    }
    pub fn get_with_definition(&self, name: &str, definition: &ComplexDefinition<T>) -> T {
        match self.map.get(name) {
            Some(x) => {
                match x {
                    &ComplexValue::Value(ref v) => v.clone(),
                    &ComplexValue::Func(ref f) => f.calculate(definition),

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
