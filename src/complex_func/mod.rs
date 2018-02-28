extern crate num_complex;
extern crate regex;

use self::regex::Regex;
use std::fmt;
#[derive(Debug)]
enum ComplexNodeType {
    Add,
    Sub,
    Mul,
    Div,
    Scalar(f64),
    String(String),
}
#[derive(Debug)]
pub struct ComplexNode {
    t: ComplexNodeType,
    left: Option<Box<ComplexNode>>,
    right: Option<Box<ComplexNode>>,
}
impl ComplexNode {
    fn to_string(&self) -> String {
        match self.t {
            ComplexNodeType::Add => String::from("Add"),
            ComplexNodeType::Sub => String::from("Sub"),
            ComplexNodeType::Mul => String::from("Mul"),
            ComplexNodeType::Div => String::from("Div"),
            ComplexNodeType::Scalar(ref x) => x.to_string(),
            ComplexNodeType::String(ref x) => x.clone(),
        }
    }
}
impl fmt::Display for ComplexNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn print(n: &ComplexNode, s: &mut String, depth: u32) {
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

impl ComplexNode {
    fn addition(s: &str) -> Option<Box<ComplexNode>> {
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
    fn multiplication(s: &str) -> Option<Box<ComplexNode>> {
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
    fn letters(s: &str) -> Option<Box<ComplexNode>> {
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
    fn numerics(s: &str) -> Option<Box<ComplexNode>> {
        let regex = Regex::new(r"\-?[[:digit:]]+\.?[[:digit:]]*").unwrap();

        if regex.is_match(s) {
            let string: &str = regex.find(s).unwrap().as_str();
            let v: Vec<&str> = regex.splitn(s, 2).collect();
            return Option::Some(Box::new(ComplexNode {
                t: ComplexNodeType::Scalar(string.parse::<f64>().expect(
                    "Failed to parse numeric value",
                )),
                left: ComplexNode::parse(v[0]),
                right: ComplexNode::parse(v[1]),
            }));
        };
        return Option::None;
    }
    pub fn parse(s: &str) -> Option<Box<ComplexNode>> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        if s.as_bytes()[0] == b'(' && s.as_bytes()[s.len() - 1] == b')' {
            return ComplexNode::parse(&s[1..(s.len() - 1)]);
        }
        println!("parsing... {}", s);
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
