extern crate itertools;

use std::fmt;
use std::iter::{Iterator, Sum, Product};
// use itertools::Itertools;
// use num_complex::Complex;
// use std::ops::Add;
// use std::cell::RefCell;

pub struct D2Vec(Box<Vec<D2>>);

#[derive(Clone, Debug)]
pub enum D2 {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
    Minkowski(Box<D2>, Box<D2>),
    Scale(f32, Box<D2>),
    ScaleXY((f32, f32), Box<D2>),
    Translate((f32, f32), Box<D2>),
    Rotate(f32, Box<D2>),
    Hull(Box<Vec<D2>>),
    Intersection(Box<Vec<D2>>),
    Union(Box<Vec<D2>>),
}

pub fn indent(shape: &D2) -> String {
    format!("{}", shape).replace("\n", "\n  ")
}

impl D2 {
    pub fn add(self, other: D2) -> D2 {
        D2::Union(Box::new(vec![self, other]))
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

    pub fn rotate_vec(&self, theta: f32, n: u32) -> Vec<D2> {
        (0..n).map(move |ii| self.rotate(theta * ii as f32)).collect::<Vec<_>>()
    }

    pub fn translate_vec(&self, x: f32, y: f32, n: u32) -> Vec<D2> {
        (0..n).map(move |ii| self.translate(x * ii as f32, y * ii as f32)).collect::<Vec<_>>()
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
        D2::Union(Box::new(iter.collect::<Vec<Self>>()))
    }
}

impl Product for D2 {
    fn product<I>(iter: I) -> Self
      where 
        I: Iterator<Item = Self>
    {
        D2::Intersection(Box::new(iter.collect::<Vec<Self>>()))
    }
}
/*
trait Union {
    fn union(self) -> D2;
}

impl Union for Vec<D2> {
    fn union(self) -> D2 {
        D2::Union(Box::new(self))
    }
}
*/
trait Combinable {
    fn union(self) -> D2;
    fn hull(self) -> D2;
}

impl Combinable for Vec<D2> {
    fn union(self) -> D2 {
        D2::Union(Box::new(self))
    }
    fn hull(self) -> D2 {
        D2::Hull(Box::new(self))
    }
}

/*
impl Vec<D2> {
    fn union(self) -> D2 {
        D2::Union(Box::new(self))
    }
}

*/
/*
pub trait Union {
    fn union<I: Iterator<Item=D2> + ?Sized>(self) -> D2;
    // fn union<I>(iter: I) -> Self
    // where
        // I: Iterator<Item = A>;
}

impl<T: Iterator<Item = D2>> Union for T {
    fn union<I>(self) -> D2
      where 
        I: Iterator<Item = D2>
    {
        D2::Union(Box::new(self.collect::<Vec<D2>>()))
    }
}

// pub struct D2Iter<I: Iterator<Item = D2>> {
    // iter: I,
// }

// pub struct D2Iter {
    // iter: Box<dyn Iterator<Item = D2>>,
// }

// impl<I> D2Iter<I> {
// impl<I> D2Iter<I> {
// impl D2Iter {
    // pub fn union(self) -> D2 {
        // D2::Union(Box::new(self.iter.collect::<Vec<_>>()))
    // }
// }
*/

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
            D2::Union(v) => write!(f,
                "union() {{\n  {}\n}}", v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Hull(v) => write!(f,
                "hull() {{\n  {}\n}}", v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Intersection(v) => write!(f,
                "intersection() {{\n  {}\n}}", v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Minkowski(a,b) => write!(f,
                "minkowski() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
        }
    }
}

trait SCAD {
    fn scad(&self) -> String;
}

impl SCAD for D2 {
    fn scad(&self) -> String {
        match &self {
            D2::Circle(r) => format!("circle(r = {});", r),
            D2::Square(size) => format!("square(size = {});", size),
            D2::Rectangle(x, y) => format!("square(size = [{}, {}]);", x, y),
            D2::Translate((x,y), shape) => format!("translate(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            D2::Rotate(theta, shape) => format!("rotate({}) {{\n  {}\n}}", theta, indent(shape)),
            D2::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s, indent(shape)),
            D2::ScaleXY((x,y), shape) => format!("scale(v = [{}, {}]) {{\n  {}\n}}", x,y, indent(shape)),
            D2::Union(v) => format!( "union() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Hull(v) => format!("hull() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Minkowski(a,b) => format!("minkowski() {{\n  {}\n  {}\n}}", indent(a), indent(b)),
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    const C5: D2 = D2::Circle(5.);
    const S9: D2 = D2::Square(9.);

    #[test]
    fn test_circle() {
        assert_eq!(C5.scad(), "circle(r = 5);");
    }

    #[test]
    fn test_square() {
        assert_eq!(S9.scad(), "square(size = 9);");
    }

    #[test]
    fn test_add() {
        assert_eq!(C5.add(S9).scad(),
        "union() {\n  circle(r = 5);\n  square(size = 9);\n}");
    }

    #[test]
    fn test_translate_iter() {
        assert_eq!(C5.translate_iter(1.,2.,4).sum::<D2>().scad(),
            "union() {\n  translate(v = [0, 0]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(r = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(r = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_iter() {
        assert_eq!(S9.rotate_iter(20., 4).sum::<D2>().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(S9.rotate_iter(20., 4).product::<D2>().scad(),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(S9.rotate_iter(20., 4).collect::<Vec<D2>>().union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_unionvec() {
        assert_eq!(S9.rotate_vec(20., 4).union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }
/*
    #[test]
    fn test_hullvec() {
        assert_eq!(format!("{}", S9.rotate_vec(20., 4).hull()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }
    */
}
