extern crate itertools;

use std::fmt;
use std::iter::{Iterator, Sum, Product};
// use itertools::Itertools;

pub trait D2Iterator : Iterator {
    fn hull(self: Self) -> D2 where Self: Iterator<Item = D2>;
    fn union(self: Self) -> D2 where Self: Iterator<Item = D2>;
    fn intersection(self: Self) -> D2 where Self: Iterator<Item = D2>;
}

impl<T: Iterator<Item=D2>> D2Iterator for T {
    fn hull(self: Self) -> D2 {
        D2::Hull(Box::new(self.collect::<Vec<D2>>()))
    }

    fn union(self: Self) -> D2 {
        D2::Union(Box::new(self.collect::<Vec<D2>>()))
    }

    fn intersection(self: Self) -> D2 {
        D2::Intersection(Box::new(self.collect::<Vec<D2>>()))
    }
}

#[derive(Clone, Debug)]
pub struct X(pub f32);

#[derive(Clone, Debug)]
pub struct XY(pub f32, pub f32);

#[derive(Clone, Debug)]
pub enum D2 {
    Circle(X),
    Square(X),
    Rectangle(XY),
    Scale(X, Box<D2>),
    ScaleXY(XY, Box<D2>),
    Translate(XY, Box<D2>),
    Rotate(X, Box<D2>),
    Hull(Box<Vec<D2>>),
    Intersection(Box<Vec<D2>>),
    Union(Box<Vec<D2>>),
    Minkowski(Box<D2>, Box<D2>),
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

    pub fn translate(&self, xy: XY) -> D2 {
        // TODO: use match to combine translate of translate
        D2::Translate(xy, Box::new(self.clone()))
    }

    pub fn translate_iter<'a>(&'a self, xy: XY, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.translate(XY(xy.0 * ii as f32, xy.1 * ii as f32)))
    }

    pub fn rotate(&self, theta: X) -> D2 {
        match self {
            D2::Rotate(X(phi), d2) => D2::Rotate(X(phi + theta.0), d2.clone()),
            _ => D2::Rotate(theta, Box::new(self.clone())),
        }
    }

    pub fn rotate_iter<'a>(&'a self, theta: X, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(X(theta.0 * ii as f32)))
    }

    pub fn translate_vec(&self, xy: XY, n: u32) -> Vec<D2> {
        (0..n).map(move |ii| self.translate(XY(xy.0 * ii as f32, xy.1 * ii as f32))).collect::<Vec<_>>()
    }

    pub fn scale(self, s: X) -> D2 {
        D2::Scale(s, Box::new(self))
    }

    pub fn scale_xy(self, xy: XY) -> D2 {
        D2::ScaleXY(xy, Box::new(self))
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

impl fmt::Display for D2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            D2::Circle(r) => write!(f, "circle(r = {});", r.0),
            D2::Square(size) => write!(f, "square(size = {});", size.0),
            D2::Rectangle(xy) => write!(f, "square(size = [{}, {}]);", xy.0, xy.1),
            D2::Translate(xy, shape) => write!(f, 
                "translate(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
            D2::Rotate(theta, shape) => write!(f, 
                "rotate({}) {{\n  {}\n}}", theta.0, indent(shape)),
            D2::Scale(s, shape) => write!(f,
                "scale(v = {}) {{\n  {}\n}}", s.0, indent(shape)),
            D2::ScaleXY(xy, shape) => write!(f,
                "scale(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
            D2::Union(v) => write!(f,
                "union() {{\n  {}\n}}", v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            // D2::Hull(v) => write!(f,
                // "hull() {{\n  {}\n}}", v.0.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            // D2::Hull2(v) => write!(f,
                // "hull() {{\n  {}\n}}", v.0.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
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
            D2::Circle(r) => format!("circle(r = {});", r.0),
            D2::Square(size) => format!("square(size = {});", size.0),
            D2::Rectangle(xy) => format!("square(size = [{}, {}]);", xy.0, xy.1),
            D2::Translate(xy, shape) => format!("translate(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
            D2::Rotate(theta, shape) => format!("rotate({}) {{\n  {}\n}}", theta.0, indent(shape)),
            D2::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s.0, indent(shape)),
            D2::ScaleXY(xy, shape) => format!("scale(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
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

    const C5: D2 = D2::Circle(X(5.));
    const S9: D2 = D2::Square(X(9.));

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
        assert_eq!(C5.translate_iter(XY(1.,2.),4).union().scad(),
            "union() {\n  translate(v = [0, 0]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(r = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(r = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_iter() {
        assert_eq!(S9.rotate_iter(X(20.), 4).sum::<D2>().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(S9.rotate_iter(X(20.), 4).product::<D2>().scad(),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(S9.rotate_iter(X(20.), 4).union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_hull() {
        assert_eq!(format!("{}", S9.rotate_iter(X(20.), 4).hull()),
            "hull() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_union() {
        assert_eq!(format!("{}", S9.rotate_iter(X(20.), 4).union()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_intersection() {
        assert_eq!(format!("{}", S9.rotate_iter(X(20.), 4).intersection()),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate_rotate() {
        assert_eq!(format!("{}", S9.rotate_iter(X(20.), 4).map(move |x| x.rotate(X(10.))).hull()),
            "hull() {\n  rotate(10) {\n    square(size = 9);\n  }\n  rotate(30) {\n    square(size = 9);\n  }\n  rotate(50) {\n    square(size = 9);\n  }\n  rotate(70) {\n    square(size = 9);\n  }\n}"
        );
    }
}
