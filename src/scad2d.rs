extern crate itertools;

use std::fmt;
use std::iter::{Iterator, Sum, Product};
pub use std::f32::consts::PI;
// use itertools::Itertools;
// const MAX: f32 = f32::MAX / 100.;
const MAX: f32 = 1000.;

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

#[derive(Clone, Copy, Debug)]
pub struct X(pub f32);

#[derive(Clone, Debug)]
pub struct XY(pub f32, pub f32);

#[derive(Clone, Debug)]
pub enum Aim {
    N, S, E, W,
    Angle(X),
}

#[derive(Clone, Debug)]
pub enum D2 {
    Circle(X),
    Square(X),
    Rectangle(XY),
    HalfPlane(Aim),
    Scale(X, Box<D2>),
    ScaleXY(XY, Box<D2>),
    Translate(XY, Box<D2>),
    Mirror(XY, Box<D2>),
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

    pub fn add_map<F>(self, f: F) -> D2 where F: Fn(D2) -> D2 {
        D2::Union(Box::new(vec![self.clone(), f(self)]))
    }

    pub fn hull(self, other: D2) -> D2 {
        D2::Hull(Box::new(vec![self, other]))
    }

    pub fn intersection(self, other: D2) -> D2 {
        D2::Intersection(Box::new(vec![self, other]))
    }

    pub fn minkowski(self, other: D2) -> D2 {
        D2::Minkowski(Box::new(self), Box::new(other))
    }

    pub fn translate(&self, xy: XY) -> D2 {
        // TODO: Is clone needed here?
        match self {
            D2::Translate(XY(a, b), d2) => D2::Translate(XY(xy.0+a, xy.1+b), d2.clone()),
            _ => D2::Translate(xy, Box::new(self.clone())),
        }
    }

    pub fn mirror(&self, xy: XY) -> D2 {
        D2::Mirror(xy, Box::new(self.clone()))
    }

    pub fn iter_translate<'a>(&'a self, xy: XY, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.translate(XY(xy.0 * ii as f32, xy.1 * ii as f32)))
    }

    pub fn rotate(&self, theta: X) -> D2 {
        match self {
            D2::Rotate(X(phi), d2) => D2::Rotate(X(phi + theta.0), d2.clone()),
            _ => D2::Rotate(theta, Box::new(self.clone())),
        }
    }

    pub fn iter_rotate<'a>(&'a self, theta: X, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(X(theta.0 * ii as f32)))
    }

    pub fn iter_rotate_equal<'a>(&'a self, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(X(360./(n as f32) * ii as f32)))
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
        write!(f, "{}", &self.scad())
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
            D2::HalfPlane(aim) => format!("{}",
                match aim {
                    Aim::N => D2::Square(X(MAX)).translate(XY(-MAX/2., 0.)),
                    Aim::S => D2::Square(X(MAX)).translate(XY(-MAX/2., -MAX)),
                    Aim::E => D2::Square(X(MAX)).translate(XY(0., -MAX/2.)),
                    Aim::W => D2::Square(X(MAX)).translate(XY(-MAX, -MAX/2.)),
                    Aim::Angle(theta) => D2::Square(X(MAX)).translate(XY(0., -MAX/2.)).rotate(*theta),
                }),
            D2::Translate(xy, shape) => format!("translate(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
            D2::Mirror(xy, shape) => format!("mirror(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
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
    fn test_iter_translate() {
        assert_eq!(C5.iter_translate(XY(1.,2.),4).union().scad(),
            "union() {\n  translate(v = [0, 0]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(r = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(r = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate() {
        assert_eq!(S9.iter_rotate(X(20.), 4).sum::<D2>().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(S9.iter_rotate(X(20.), 4).product::<D2>().scad(),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(S9.iter_rotate(X(20.), 4).union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_hull() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).hull()),
            "hull() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_union() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).union()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_intersection() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).intersection()),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate_rotate() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).map(move |x| x.rotate(X(10.))).hull()),
            "hull() {\n  rotate(10) {\n    square(size = 9);\n  }\n  rotate(30) {\n    square(size = 9);\n  }\n  rotate(50) {\n    square(size = 9);\n  }\n  rotate(70) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_translate_translate() {
        assert_eq!(C5.iter_translate(XY(1.,2.),4).map(move |x| x.translate(XY(-1., -1.))).union().scad(),
            "union() {\n  translate(v = [-1, -1]) {\n    circle(r = 5);\n  }\n  translate(v = [0, 1]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 3]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 5]) {\n    circle(r = 5);\n  }\n}"
        );
    }

}
