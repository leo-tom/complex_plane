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

pub mod complex_definition;

use self::regex::Regex;
use std::fmt;
use complex_func::complex_definition::ComplexDefinition;
use num_complex::Complex;
use std::error::Error;
#[derive(Debug)]
pub enum CalculationError {
    ValueNotDefined(String),
    Unknown(String),
}
#[derive(Debug)]
pub enum ParseError {
    Unknown(String),
}
impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CalculationError::ValueNotDefined(ref s) => {
                write!(f, "{} is not defined as function/value", s)
            }
            &CalculationError::Unknown(ref s) => write!(f, "CalculationError : {}", s),
        }

    }
}
impl Error for CalculationError {
    fn description(&self) -> &str {
        "CalculationError"
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::Unknown(ref s) => write!(f, "{}", s),
        }

    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        "ParseError"
    }
}
#[derive(Debug)]
enum ComplexNodeType<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Scalar(Complex<T>),
    String(String),
    Vector(Vec<ComplexNode<T>>),
}
#[derive(Debug)]
pub struct ComplexNode<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    t: ComplexNodeType<T>,
    left: Option<Box<ComplexNode<T>>>,
    right: Option<Box<ComplexNode<T>>>,
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Default
    for ComplexNode<T> {
    fn default() -> Self {
        ComplexNode {
            t: ComplexNodeType::Scalar(Complex::new(T::zero(), T::zero())),
            left: None,
            right: None,
        }
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> Clone
    for ComplexNode<T> {
    fn clone(&self) -> Self {
        let t: ComplexNodeType<T> = match self.t {
            ComplexNodeType::Scalar(ref x) => ComplexNodeType::Scalar(x.clone()),
            ComplexNodeType::String(ref x) => ComplexNodeType::String(x.clone()),
            ComplexNodeType::Add => ComplexNodeType::Add,
            ComplexNodeType::Sub => ComplexNodeType::Sub,
            ComplexNodeType::Mul => ComplexNodeType::Mul,
            ComplexNodeType::Div => ComplexNodeType::Div,
            ComplexNodeType::Pow => ComplexNodeType::Pow,
            ComplexNodeType::Vector(ref x) => ComplexNodeType::Vector(x.clone()),
        };
        let left: Option<Box<ComplexNode<T>>> = match self.left {
            Some(ref x) => Some(x.clone()),
            _ => None,
        };
        let right: Option<Box<ComplexNode<T>>> = match self.right {
            Some(ref x) => Some(x.clone()),
            _ => None,
        };
        ComplexNode {
            t: t,
            left: left,
            right: right,
        }
    }
}

impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
    ComplexNode<T> {
    pub fn fromc(c: Complex<T>) -> Self {
        ComplexNode {
            t: ComplexNodeType::Scalar(c.clone()),
            left: None,
            right: None,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self.t {
            ComplexNodeType::Vector(_) => true,
            _ => false,
        }
    }
    pub fn get_vec(&self) -> Vec<ComplexNode<T>> {
        match self.t {
            ComplexNodeType::Vector(ref v) => v.clone(),
            _ => vec![self.clone()],
        }
    }
    pub fn is_const(&self) -> bool {
        if let ComplexNodeType::String(_) = self.t {
            return false;
        }
        let left = match self.left {
            Some(ref x) => x.is_const(),
            _ => true,
        };
        if !left {
            return false;
        }
        let right = match self.right {
            Some(ref x) => x.is_const(),
            _ => true,
        };
        return right;
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
    ComplexNode<T> {
    pub fn to_string(&self) -> String {
        self.get_name()
    }
    pub fn get_name(&self) -> String {
        match self.t {
            ComplexNodeType::Add => String::from("Add"),
            ComplexNodeType::Sub => String::from("Sub"),
            ComplexNodeType::Mul => String::from("Mul"),
            ComplexNodeType::Div => String::from("Div"),
            ComplexNodeType::Pow => String::from("Pow"),
            ComplexNodeType::Scalar(ref x) => x.re.to_f64().unwrap().to_string(),
            ComplexNodeType::String(ref x) => x.clone(),
            ComplexNodeType::Vector(ref x) => {
                let mut s = String::new();
                for v in x {
                    s.push_str(&v.to_string());
                }
                s
            }
        }
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> fmt::Display
    for ComplexNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		fn print<T: num::traits::Num + num_traits::ToPrimitive+ num_traits::FromPrimitive+ Clone>
(n: &ComplexNode<T>, s: &mut String, depth: u32){
            let pushed_string = n.to_string();
            s.push_str(pushed_string.as_str());
            if let Some(ref x) = n.right {
                s.push('_');
                print(x, s, depth + 1 + pushed_string.len() as u32);
            }
            if let Some(ref x) = n.left {
                s.push('\n');
                for _ in 0..depth {
                    s.push(' ');
                }
                s.push('|');
                s.push('\n');
                for _ in 0..depth {
                    s.push(' ');
                }
                print(x, s, depth);
            }
        }
        let mut result: String = String::new();
        print(self, &mut result, 0);
        write!(f, "{}", result)
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
    ComplexNode<T> {
    pub fn calculate(
        &self,
        definition: &ComplexDefinition<T>,
    ) -> Result<Complex<T>, CalculationError> {
        match self.t {
            ComplexNodeType::Scalar(ref x) => {
                let left = match self.left {
                    Some(ref y) => y.calculate(definition)?,
                    _ => Complex::new(T::one(), T::zero()),
                };
                let right = match self.right {
                    Some(ref y) => y.calculate(definition)?,
                    _ => Complex::new(T::one(), T::zero()),
                };
                Ok(left * x.clone() * right)
            }
            ComplexNodeType::Vector(ref x) => x[0].calculate(definition),
            ComplexNodeType::String(ref x) => {
                if definition.is_function(x) {
                    match self.right {
                        Some(ref right) => definition.call(x, right),
                        _ => {
                            /*call without argument*/
                            definition.call(
                                x,
                                &ComplexNode {
                                    t: ComplexNodeType::Add,
                                    right: None,
                                    left: None,
                                },
                            )
                        }
                    }
                } else {
                    definition.get(x)?.calculate(&definition)
                }
            }
            _ => {
                let left = match self.left {
                    Some(ref x) => x.calculate(definition)?,
                    _ => Complex::<T>::from(T::zero()),
                };
                let right = match self.right {
                    Some(ref x) => x.calculate(definition)?,
                    _ => Complex::from(T::zero()),
                };
                match self.t {
                    ComplexNodeType::Add => Ok(left + right),
                    ComplexNodeType::Sub => Ok(left - right),
                    ComplexNodeType::Mul => Ok(left * right),
                    ComplexNodeType::Div => Ok(left / right),
                    ComplexNodeType::Pow => {
                        let left =
                            Complex::new(left.re.to_f64().unwrap(), left.im.to_f64().unwrap());
                        let right =
                            Complex::new(right.re.to_f64().unwrap(), right.im.to_f64().unwrap());
                        let ans = left.powc(right);
                        let ans = Complex::new(
                            T::from_f64(ans.re).unwrap(),
                            T::from_f64(ans.im).unwrap(),
                        );
                        Ok(ans)
                    }
                    _ => Err(CalculationError::Unknown(
                        "You should not be seeing me. Report it!".to_owned(),
                    )),
                }
            }
        }

    }
    fn get_left(&self) -> &Self {
        match self.left {
            Some(ref x) => x.get_left(),
            _ => self,
        }
    }
    fn get_right(&self) -> &Self {
        match self.right {
            Some(ref x) => x.get_right(),
            _ => self,
        }
    }
    fn append_left(&mut self, v: Self) {
        match self.left {
            Some(ref mut x) => x.append_left(v),
            _ => self.left = Some(Box::new(v)),
        }
    }
    fn append_right(&mut self, v: Self) {
        match self.right {
            Some(ref mut x) => x.append_right(v),
            _ => self.right = Some(Box::new(v)),
        }
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
    ComplexNode<T> {
    fn addition(s: &str) -> Option<Box<ComplexNode<T>>> {
        let v: Vec<&str> = s.splitn(2, '+').collect();
        if v.len() > 1 {
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Add,
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        let v: Vec<&str> = s.splitn(2, '-').collect();
        if v.len() > 1 {
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Sub,
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        }
        return Option::None;
    }
    fn multiplication(s: &str) -> Option<Box<ComplexNode<T>>> {
        let v: Vec<&str> = s.splitn(2, '*').collect();
        if v.len() > 1 {
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Mul,
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        let v: Vec<&str> = s.splitn(2, '/').collect();
        if v.len() > 1 {
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Div,
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        }
        let v: Vec<&str> = s.splitn(2, '^').collect();
        if v.len() > 1 {
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Pow,
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        }
        return Option::None;
    }
    fn letters(s: &str) -> Option<Box<ComplexNode<T>>> {
        let regex = Regex::new("[[:alpha:]]+[[:alnum:]]*").unwrap();
        if regex.is_match(s) {
            let string: String = regex.find(s).unwrap().as_str().to_string();
            let v: Vec<&str> = regex.splitn(s, 2).collect();
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::String(string),
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        return Option::None;
    }
    fn numerics(s: &str) -> Option<Box<ComplexNode<T>>> {
        let regex = Regex::new(r"\-?[[:digit:]]+\.?[[:digit:]]*").unwrap();

        if regex.is_match(s) {
            let string: &str = regex.find(s).unwrap().as_str();
            let v: Vec<&str> = regex.splitn(s, 2).collect();
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Scalar(Complex::from(
                    T::from_f64(string.parse::<f64>().expect(
                        "Failed to parse numeric value",
                    )).unwrap(),
                )),
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        return Option::None;
    }
    fn vector(s: &str) -> Option<Box<ComplexNode<T>>> {
        if s.find(',').is_some() {
            let mut v: Vec<ComplexNode<T>> = Vec::new();
            let splited: Vec<&str> = s.split(',').collect();
            for piece in splited {
                v.push(*ComplexNode::<T>::parse(piece).unwrap());
            }
            return Some(Box::new(ComplexNode {
                t: ComplexNodeType::Vector(v),
                left: None,
                right: None,
            }));
        }
        return None;
    }
    fn brakets(s: &str) -> Option<Box<ComplexNode<T>>> {
        let regex = Regex::new(r"\(.*\)").unwrap();
        if regex.is_match(s) {
            let splited: Vec<&str> = s.splitn(2, '(').collect();
            let mut counter = 1;
            let mut index = 0;
            let mut center = String::new();
            for c in splited[1].chars() {
                index += 1;
                if c == '(' {
                    counter += 1;
                } else if c == ')' {
                    counter -= 1;
                }
                center.push(c);
                if counter <= 0 {
                    center.pop();
                    break;
                }
            }
            let left = splited[0];
            let (_, right) = splited[1].split_at(index);
            let left = ComplexNode::<T>::parse(left);
            let right = ComplexNode::<T>::parse(right);
            let center = match Self::vector(center.as_str()) {
                Some(x) => Some(x),
                _ => ComplexNode::<T>::parse(center.as_str()),
            };

            if left.is_some() && right.is_some() {
                let mut right = right.unwrap();
                let mut left = left.unwrap();
                let rl_level = match right.get_left().t {
                    ComplexNodeType::Add => 0,
                    ComplexNodeType::Sub => 0,
                    _ => 1,
                };

                let lr_level = match left.get_right().t {
                    ComplexNodeType::Add => 0,
                    ComplexNodeType::Sub => 0,
                    _ => 1,
                };
                if lr_level > rl_level {
                    // left*(center)+right
                    left.append_right(*center.unwrap());
                    right.append_left(*left);
                    return Some(right);
                } else if lr_level < rl_level {
                    // left+(center)*right
                    right.append_left(*center.unwrap());
                    left.append_right(*right);
                    return Some(left);
                } else {
                    // left+(center)+right
                    right.append_left(*center.unwrap());
                    left.append_right(*right);
                    return Some(left);
                }
            } else if left.is_some() && right.is_none() {
                let mut left = left.unwrap();
                left.append_right(*center.unwrap());
                return Some(left);
            } else if left.is_none() && right.is_some() {
                let mut right = right.unwrap();
                right.append_left(*center.unwrap());
                return Some(right);
            } else {
                /*left.is_none() && right.is_none()*/
                return center;
            }


        }
        return None;
    }
    pub fn parse(s: &str) -> Option<Box<ComplexNode<T>>> {
        let s = s.trim();

        if s.is_empty() {
            return None;
        }
        //println!("parsing... {}", s);
        match ComplexNode::brakets(s) {
            x @ Some(_) => return x,
            None => (),
        }
        match ComplexNode::addition(s) {
            x @ Some(_) => return x,
            None => (),
        }
        match ComplexNode::multiplication(s) {
            x @ Some(_) => return x,
            None => (),
        }
        match ComplexNode::numerics(s) {
            x @ Some(_) => return x,
            None => (),
        }
        match ComplexNode::letters(s) {
            x @ Some(_) => return x,
            None => (),
        }

        return Option::None;
    }
}
