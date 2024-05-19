extern crate itertools;

use std::fmt;
use itertools::Itertools;
use std::iter::Sum;
// use std::ops::Add;
use std::cell::RefCell;

// pub struct D2_iter(Vec<D2>);

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
    Rotate(f32, Box<D2>),
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

    pub fn translate(&self, x: f32, y: f32) -> D2 {
        D2::Translate((x,y), Box::new(self.clone()))
    }

    pub fn translate_iter<'a>(&'a self, x: f32, y: f32, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.translate(x * ii as f32, y * ii as f32))
    }

    pub fn rotate(&self, theta: f32) -> D2 {
        D2::Rotate(theta, Box::new(self.clone()))
    }

    pub fn rotate_iter<'a>(&'a self, theta: f32, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(theta * ii as f32))
    }

    pub fn scale(self, s: f32) -> D2 {
        D2::Scale(s, Box::new(self))
    }

    pub fn scale_xy(self, x: f32, y: f32) -> D2 {
        D2::ScaleXY((x,y), Box::new(self))
    }
}

impl Sum for D2 {
    fn sum<I>(iter: I) -> Self
      where 
        I: Iterator<Item = Self>
    {
        D2::Union(RefCell::new(iter.collect::<Vec<Self>>()))
    }
}

// impl D2_iter {
    // pub fn union(&self) -> D2 {
        // D2::Union(RefCell::new(self.collect::<Vec<_>>()))
    // }
// }


impl fmt::Display for D2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            D2::Circle(r) => write!(f, "circle(r = {});", r),
            D2::Square(size) => write!(f, "square(size = {});", size),
            D2::Rectangle(x, y) => write!(f, "square(size = [{}, {}]);", x, y),
            D2::Translate((x,y), shape) => write!(f, 
                "translate(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            D2::Rotate(theta, shape) => write!(f, 
                "rotate({}) {{\n  {}\n}}", theta, indent(shape)),
            D2::Scale(s, shape) => write!(f,
                "scale(v = {}) {{\n  {}\n}}", s, indent(shape)),
            D2::ScaleXY((x,y), shape) => write!(f,
                "scale(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            // D2::Union(a,b) => write!(f,
                // "union() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
            // D2::Union(v) => write!(f,
                // "union() {{\n  {}\n  {}\n}}", indent(&v[0]), indent(&v[1])),
            D2::Union(v) => write!(f,
                "union() {{\n  {}\n}}", v.borrow().iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Minkowski(a,b) => write!(f,
                "minkowski() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    // lazy_static! {
        // static ref c5: D2 = D2::Circle(5.);
    // }
    const c5: D2 = D2::Circle(5.);
    const s9: D2 = D2::Square(9.);

    #[test]
    fn test_circle() {
        assert_eq!(format!("{}", c5), "circle(r = 5);");
    }

    #[test]
    fn test_square() {
        assert_eq!(format!("{}", s9), "square(size = 9);");
    }

    #[test]
    fn test_add() {
        assert_eq!(format!("{}", c5.add(s9)),
        "union() {\n  circle(r = 5);\n  square(size = 9);\n}");
    }

    #[test]
    fn test_union() {
        assert_eq!(format!("{}", c5.add(s9)),
        "union() {\n  circle(r = 5);\n  square(size = 9);\n}");
    }

    #[test]
    fn test_translate_iter() {
        assert_eq!(format!("{}", c5.translate_iter(1.,2.,4).sum::<D2>()),
            "union() {\n  translate(v = [0, 0]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(r = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(r = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_iter() {
        assert_eq!(format!("{}", s9.rotate_iter(20., 4).sum::<D2>()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }
}
