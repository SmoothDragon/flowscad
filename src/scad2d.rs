extern crate itertools;

use std::fmt;
use itertools::Itertools;
// use std::ops::Add;
use std::cell::RefCell;


#[derive(Clone, Debug)]
pub enum D2 {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
    // Union(Box<D2>, Box<D2>),
    // Union(Box<Vec<D2>>),
    Union(RefCell<Vec<D2>>),
    Minkowski(Box<D2>, Box<D2>),
    Scale(f32, Box<D2>),
    ScaleXY((f32, f32), Box<D2>),
    Translate((f32, f32), Box<D2>),
}

pub fn indent(shape: &D2) -> String {
    format!("{}", shape).replace("\n", "\n  ")
}

impl D2 {
    pub fn add(self, other: D2) -> D2 {
        // D2::Union(Box::new(self), Box::new(other))
        // D2::Union(Box::new(vec![self, other]))
        D2::Union(RefCell::new(vec![self, other]))
    }
    pub fn minkowski(self, other: D2) -> D2 {
        D2::Minkowski(Box::new(self), Box::new(other))
    }
    pub fn translate(self, x: f32, y: f32) -> D2 {
        D2::Translate((x,y), Box::new(self))
    }
    pub fn scale(self, s: f32) -> D2 {
        D2::Scale(s, Box::new(self))
    }
    pub fn scale_xy(self, x: f32, y: f32) -> D2 {
        D2::ScaleXY((x,y), Box::new(self))
    }
}

impl fmt::Display for D2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            D2::Circle(r) => write!(f, "circle(r = {});", r),
            D2::Square(size) => write!(f, "square(size = {});", size),
            D2::Rectangle(x, y) => write!(f, "square(size = [{}, {}]);", x, y),
            D2::Translate((x,y), shape) => write!(f, 
                "translate(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            D2::Scale(s, shape) => write!(f,
                "scale(v = {}) {{\n  {}\n}}", s, indent(shape)),
            D2::ScaleXY((x,y), shape) => write!(f,
                "scale(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            // D2::Union(a,b) => write!(f,
                // "union() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
            // D2::Union(v) => write!(f,
                // "union() {{\n  {}\n  {}\n}}", indent(&v[0]), indent(&v[1])),
            D2::Union(v) => write!(f,
                "union() {{\n  {}\n}}", v.borrow().iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("\n")),
            D2::Minkowski(a,b) => write!(f,
                "minkowski() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(format!("{}", D2::Circle(5.)), "circle(r = 5);");
    }

    #[test]
    fn test_square() {
        assert_eq!(format!("{}", D2::Square(5.)), "square(size = 5);");
    }

    #[test]
    fn test_union() {
        assert_eq!(format!("{}", D2::Circle(5.).add(D2::Square(10.))), "circle(r = 5);");
    }
}
