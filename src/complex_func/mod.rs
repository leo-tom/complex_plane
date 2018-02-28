extern crate num_complex;
extern crate regex;
extern crate num_traits;
extern crate num;


use self::regex::Regex;
use std::fmt;
use complex_plane::Plane;
use num_complex::Complex;

#[derive(Debug)]
enum ComplexNodeType<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    Add,
    Sub,
    Mul,
    Div,
    Scalar(T),
    String(String),
}
#[derive(Debug)]
pub struct ComplexNode<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> {
    t: ComplexNodeType<T>,
    left: Option<Box<ComplexNode<T>>>,
    right: Option<Box<ComplexNode<T>>>,
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone>
    ComplexNode<T> {
    fn to_string(&self) -> String {
        match self.t {
            ComplexNodeType::Add => String::from("Add"),
            ComplexNodeType::Sub => String::from("Sub"),
            ComplexNodeType::Mul => String::from("Mul"),
            ComplexNodeType::Div => String::from("Div"),
            ComplexNodeType::Scalar(ref x) => x.to_f64().unwrap().to_string(),
            ComplexNodeType::String(ref x) => x.clone(),
        }
    }
}
impl<T: num::traits::Num + num_traits::ToPrimitive + num_traits::FromPrimitive + Clone> fmt::Display
    for ComplexNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

fn print<T: num::traits::Num + num_traits::ToPrimitive+ num_traits::FromPrimitive+ Clone>(n: &ComplexNode<T>, s: &mut String, depth: u32){
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
    pub fn calculate(&self) -> T {
        let left = match self.left {
            Some(ref x) => x.calculate(),
            _ => {
                match T::from_str_radix("0", 0) {
                    Ok(y) => y,
                    _ => T::zero(),
                }
            }
        };
        let right = match self.right {
            Some(ref x) => x.calculate(),
            _ => {
                match T::from_str_radix("0", 0) {
                    Ok(y) => y,
                    _ => T::zero(),
                }
            }
        };
        match self.t {
            ComplexNodeType::Add => left + right,
            ComplexNodeType::Sub => left - right,
            ComplexNodeType::Mul => left * right,
            ComplexNodeType::Div => left / right,
            ComplexNodeType::Scalar(ref x) => x.clone(),
            _ => T::zero(),
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
                t: ComplexNodeType::Scalar(
                    T::from_f64(string.parse::<f64>().expect(
                        "Failed to parse numeric value",
                    )).unwrap(),
                ),
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        return Option::None;
    }


    fn brakets(s: &str) -> Option<Box<ComplexNode<T>>> {
        let regex = Regex::new(r"\(.*\)").unwrap();
        if regex.is_match(s) {
            let string = regex.find(s).unwrap().as_str();
            let v: Vec<&str> = regex.splitn(s, 2).collect();
            if s == string {
                return Self::brakets(s.trim_left_matches('(').trim_right_matches(')'));
            }
            println!("{} {:?}", string, v);
            let left = ComplexNode::<T>::parse(v[0]);
            let right = ComplexNode::<T>::parse(v[1]);
            let center = ComplexNode::<T>::parse(string);

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
                    right.append_right(*left);
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
        println!("parsing... {}", s);
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
