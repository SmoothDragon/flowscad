//! Create OpenSCAD files using Rust.
extern crate itertools;

use std::fmt;
use std::iter::{Iterator, Sum, Product};
pub use std::f64::consts::PI;
use num_traits::Num;
use lazy_static::lazy_static;

// use itertools::Itertools;
const MAX: f64 = f64::MAX / 100.;
// const MAX: f64 = 1000.;


/// Methods for creating an SCAD object from an iterator of SCAD objects.
pub trait DIterator<T> : Iterator<Item=T> {
    fn hull(self: Self) -> T where Self: Iterator<Item = T>;
    fn union(self: Self) -> T where Self: Iterator<Item = T>;
    fn intersection(self: Self) -> T where Self: Iterator<Item = T>;
    fn minkowski(self: Self) -> T where Self: Iterator<Item = T>;
}


impl<T: Iterator<Item=D2>> DIterator<D2> for T {
    fn hull(self: Self) -> D2 {
        D2::Hull(Box::new(self.collect::<Vec<D2>>()))
    }

    fn union(self: Self) -> D2 {
        D2::Union(Box::new(self.collect::<Vec<D2>>()))
    }

    fn intersection(self: Self) -> D2 {
        D2::Intersection(Box::new(self.collect::<Vec<D2>>()))
    }

    fn minkowski(self: Self) -> D2 {
        D2::Minkowski(Box::new(self.collect::<Vec<D2>>()))
    }
}

impl<T: Iterator<Item=D3>> DIterator<D3> for T {
    fn hull(self: Self) -> D3 {
        D3::Hull(Box::new(self.collect::<Vec<D3>>()))
    }

    fn union(self: Self) -> D3 {
        D3::Union(Box::new(self.collect::<Vec<D3>>()))
    }

    fn intersection(self: Self) -> D3 {
        D3::Intersection(Box::new(self.collect::<Vec<D3>>()))
    }

    fn minkowski(self: Self) -> D3 {
        D3::Minkowski(Box::new(self.collect::<Vec<D3>>()))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct X(pub f64);

impl From<i32> for X {
    fn from(i: i32) -> X {
        X(i as f64)
    }
}

impl From<f64> for X {
    fn from(f: f64) -> X {
        X(f)
    }
}

#[derive(Clone, Debug)]
pub struct XY(pub f64, pub f64);

#[derive(Clone, Debug)]
pub struct XYZ(pub f64, pub f64, pub f64);

#[derive(Clone, Debug)]
pub enum Aim {
    N, S, E, W,
    U, D, L, R,
    Angle(X),
}

#[derive(Clone, Debug)]
pub enum Pos {
    N, S, E, W,
    NE, NW, SE, SW,
    U, D, L, R,
    Angle(X),
    Center,
}

#[derive(Clone, Debug)]
pub enum Color {
    Blue,
    Green,
    Red,
}

#[derive(Clone, Debug)]
pub enum D3 {
    Cube(X),
    Cylinder(X, X),
    Box(XYZ),
    Translate(XYZ, Box<D3>),
    Rotate(XYZ, Box<D3>),
    LinearExtrude(X, Box<D2>),
    Hull(Box<Vec<D3>>),
    Intersection(Box<Vec<D3>>),
    Union(Box<Vec<D3>>),
    Minkowski(Box<Vec<D3>>),
    Difference(Box<D3>, Box<D3>),
}

#[derive(Clone, Debug)]
pub enum D2 {
    Circle(X),
    Square(X),
    Rectangle(XY),
    HalfPlane(Aim),
    Color(Color, Box<D2>),
    Rotate(X, Box<D2>),
    Scale(X, Box<D2>),
    ScaleXY(XY, Box<D2>),
    Translate(XY, Box<D2>),
    Mirror(XY, Box<D2>),
    Hull(Box<Vec<D2>>),
    Intersection(Box<Vec<D2>>),
    Union(Box<Vec<D2>>),
    Minkowski(Box<Vec<D2>>),
    // Minkowski(Box<D2>, Box<D2>),
}

// #[derive(Clone, Debug)]
// pub enum Geo<T> where T: D2 + D3 {
// }


pub fn indent(shape: &D2) -> String {
    format!("{}", shape).replace("\n", "\n  ")
}

pub fn indent_d3(shape: &D3) -> String {
    format!("{}", shape).replace("\n", "\n  ")
}

impl D2 {
    /// Create a circle of radius `r` centered at the origin.
    pub fn circle<T: Copy + Into<f64>>(r: T) -> D2 {
        D2::Circle(X(r.into()))
    }

    pub fn square<T: Copy + Into<f64>>(s: T) -> D2 {
        D2::Square(X(s.into()))
    }

    pub fn add(self, other: D2) -> D2 {
        match self { // Combine Unions if possible
            D2::Union(vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Union(vec)
                },
            _ => D2::Union(Box::new(vec![self, other])),
        }
    }

    pub fn add_map<F>(self, f: F) -> D2 where F: Fn(D2) -> D2 {
        self.clone().add(f(self))
        // D2::Union(Box::new(vec![self.clone(), f(self)]))
    }
    /*
    pub fn hull(self, other: D2) -> D2 {
        match self { // Combine D2 hulls if possible
            D2::Hull(vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Hull(vec)
                },
            _ => D2::Hull(Box::new(vec![self, other])),
        }
        // D2::Hull(Box::new(vec![self, other]))
    }
    */
    pub fn hull(self) -> D2 {
        // D3::Hull(Box::new(vec![self]))
        match self { // Combine Unions if possible
            D2::Union(vec) => D2::Hull(vec),
            _ => D2::Hull(Box::new(vec![self])),
        }
    }

    pub fn intersection(self, other: D2) -> D2 {
        match self { // Combine intersections if possible
            D2::Intersection(vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Intersection(vec)
                },
            _ => D2::Intersection(Box::new(vec![self, other])),
        }
        // D2::Intersection(Box::new(vec![self, other]))
    }

    pub fn minkowski(self, other: D2) -> D2 {
        match self { // Combine Minkowski sums if possible
            D2::Minkowski(vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Minkowski(vec)
                },
            _ => D2::Minkowski(Box::new(vec![self, other])),
        }
        // D2::Minkowski(Box::new(vec![self, other]))
        // D2::Minkowski(Box::new(self), Box::new(other))
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
        (0..n).map(move |ii| self.translate(XY(xy.0 * ii as f64, xy.1 * ii as f64)))
    }

    pub fn rotate(&self, theta: X) -> D2 {
        match self {
            D2::Rotate(X(phi), d2) => D2::Rotate(X(phi + theta.0), d2.clone()),
            _ => D2::Rotate(theta, Box::new(self.clone())),
        }
    }

    pub fn iter_rotate<'a>(&'a self, theta: X, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(X(theta.0 * ii as f64)))
    }

    pub fn iter_rotate_equal<'a>(&'a self, n: u32) -> impl Iterator<Item = D2> + 'a {
        (0..n).map(move |ii| self.rotate(X(360./(n as f64) * ii as f64)))
    }

    pub fn iter_square_edge<'a>(&'a self, d: X) -> impl Iterator<Item = D2> + 'a {
        vec![XY(d.0, 0.), XY(0., d.0), XY(-d.0, 0.), XY(0., -d.0)]
            .into_iter()
            .map(move |xy| self.translate(xy))
    }

    pub fn translate_vec(&self, xy: XY, n: u32) -> Vec<D2> {
        (0..n).map(move |ii| self.translate(XY(xy.0 * ii as f64, xy.1 * ii as f64))).collect::<Vec<_>>()
    }

    pub fn color(self, color_name: Color) -> D2 {
        D2::Color(color_name, Box::new(self))
    }


    pub fn scale(self, s: X) -> D2 {
        D2::Scale(s, Box::new(self))
    }

    pub fn scale_xy(self, xy: XY) -> D2 {
        D2::ScaleXY(xy, Box::new(self))
    }

    pub fn linear_extrude(&self, x: X) -> D3 {
        D3::LinearExtrude(x, Box::new(self.clone()))
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

impl fmt::Display for D3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.scad())
    }
}

trait SCAD {
    fn scad(&self) -> String;
    fn indent(&self) -> String;
}

impl SCAD for D3 {
    fn scad(&self) -> String {
        match &self {
            D3::LinearExtrude(X(h), shape) => format!("linear_extrude(height = {}) {{\n  {}\n}}", h, indent(shape)),
            D3::Cube(size) => format!("cube(size = {});", size.0),
            D3::Box(xyz) => format!("cube(size = [{}, {}, {}]);", xyz.0, xyz.1, xyz.2),
            D3::Cylinder(h, r) => format!("cylinder(h = {}, r = {});", h.0, r.0),
            D3::Union(v) => format!( "union() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Hull(v) => format!("hull() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Minkowski(v) => format!("minkowski() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            // D3::Translate(xyz, shape) => format!("translate(v = [{}, {}, {}]) {{\n  {}\n}}", xyz.0, xyz.1, xyz.2, indent_d3(shape)),
            D3::Translate(xyz, shape) => format!("translate(v = [{}, {}, {}]) {{\n  {}\n}}", xyz.0, xyz.1, xyz.2, shape.indent()),
            D3::Rotate(theta, shape) => format!("rotate([{}, {}, {}]) {{\n  {}\n}}", theta.0, theta.1, theta.2, indent_d3(shape)),
            D3::Difference(shape1, shape2) => format!("difference() {{\n  {}\n  {}\n}}", indent_d3(shape1), indent_d3(shape2)),
        }
    }
    fn indent(&self) -> String {
        self.scad().replace("\n", "\n  ")
    }

}

impl D3 {
    /// Create a cylinder of height `h` and radius `r` centered above the XY plane.
    // pub fn cylinder<T: Copy + Into<f64>>(h: T, r: T) -> D3 {
        // D3::Cylinder(X(h.into()), X(r.into()))
    // }
    pub fn cylinder(h: f64, r: f64) -> D3 {
        D3::Cylinder(X(h), X(r))
    }

    pub fn add(self, other: D3) -> D3 {
        match self { // Combine Unions if possible
            D3::Union(vec) => {
                let mut vec = vec;
                vec.push(other);
                D3::Union(vec)
                },
            _ => D3::Union(Box::new(vec![self, other])),
        }
    }

    pub fn difference(self, other: D3) -> D3 {
        D3::Difference(Box::new(self), Box::new(other))
    }

    pub fn add_map<F>(self, f: F) -> D3 where F: Fn(D3) -> D3 {
        self.clone().add(f(self))
    }


    pub fn translate(&self, xyz: XYZ) -> D3 {
        D3::Translate(xyz, Box::new(self.clone()))
    }

    pub fn rotate(&self, theta: XYZ) -> D3 {
        D3::Rotate(theta, Box::new(self.clone()))
    }

    pub fn iter_rotate<'a>(&'a self, theta: XYZ, n: u32) -> impl Iterator<Item = D3> + 'a {
        (0..n).map(move |ii| self.rotate(XYZ(theta.0 * ii as f64, theta.1 * ii as f64, theta.2 * ii as f64)))
    }

    pub fn hull(self) -> D3 {
        // D3::Hull(Box::new(vec![self]))
        match self { // Combine Unions if possible
            D3::Union(vec) => D3::Hull(vec),
            _ => D3::Hull(Box::new(vec![self])),
        }
    // pub fn hull(self, other: D3) -> D3 {
        // match self { // Combine D3 hulls if possible
            // D3::Hull(vec) => {
                // let mut vec = vec;
                // vec.push(other);
                // D3::Hull(vec)
                // },
            // _ => D3::Hull(Box::new(vec![self, other])),
        // }
    }

    pub fn intersection(self, other: D3) -> D3 {
        match self { // Combine intersections if possible
            D3::Intersection(vec) => {
                let mut vec = vec;
                vec.push(other);
                D3::Intersection(vec)
                },
            _ => D3::Intersection(Box::new(vec![self, other])),
        }
    }

    pub fn beveled_box(xyz: XYZ, bevel: f64) -> D3 {
        let x = xyz.0; 
        let y = xyz.1;
        let z = xyz.2;
        D3::Hull(Box::new(vec![
            D3::Box(XYZ(x,y-bevel*2.,z-bevel*2.)).translate(XYZ(0.,bevel,bevel)),
            D3::Box(XYZ(x-bevel*2.,y-bevel*2.,z)).translate(XYZ(bevel,bevel,0.)),
            D3::Box(XYZ(x-bevel*2.,y,z-bevel*2.)).translate(XYZ(bevel,0.,bevel)),
            ]))
    }


}

impl SCAD for D2 {
    fn scad(&self) -> String {
        match &self {
            D2::Circle(X(radius)) => format!("circle(r = {});", radius),
            D2::Square(X(size)) => format!("square(size = {});", size),
            D2::Rectangle(XY(x,y)) => format!("square(size = [{}, {}]);", x, y),
            D2::Color(color, shape) => format!("color({}) {{\n  {}\n}}", 
                match color {
                    Color::Blue => "\"blue\"",
                    Color::Green => "\"green\"",
                    Color::Red => "\"red\"",
                }
                , indent(shape)),
            D2::HalfPlane(aim) => format!("{}",
                match aim {
                    Aim::N => D2::Square(X(MAX)).translate(XY(-MAX/2., 0.)),
                    Aim::U => D2::Square(X(MAX)).translate(XY(-MAX/2., 0.)),
                    Aim::S => D2::Square(X(MAX)).translate(XY(-MAX/2., -MAX)),
                    Aim::D => D2::Square(X(MAX)).translate(XY(-MAX/2., -MAX)),
                    Aim::E => D2::Square(X(MAX)).translate(XY(0., -MAX/2.)),
                    Aim::R => D2::Square(X(MAX)).translate(XY(0., -MAX/2.)),
                    Aim::W => D2::Square(X(MAX)).translate(XY(-MAX, -MAX/2.)),
                    Aim::L => D2::Square(X(MAX)).translate(XY(-MAX, -MAX/2.)),
                    // Aim::Angle(theta) => D2::Square(X(MAX)).translate(XY(0., -MAX/2.)).rotate(*theta),
                    Aim::Angle(theta) => D2::HalfPlane(Aim::E).rotate(*theta),
                }),
            D2::Translate(XY(x,y), shape) => format!("translate(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
            D2::Mirror(XY(x,y), shape) => format!("mirror(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
            D2::Rotate(X(theta), shape) => format!("rotate({}) {{\n  {}\n}}", theta, indent(shape)),
            D2::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s.0, indent(shape)),
            D2::ScaleXY(xy, shape) => format!("scale(v = [{}, {}]) {{\n  {}\n}}", xy.0, xy.1, indent(shape)),
            D2::Union(v) => format!( "union() {{\n  {}\n}}",
                v.iter().map(|x| x.indent()).collect::<Vec<_>>().join("\n  ")),
            D2::Hull(v) => format!("hull() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Minkowski(v) => format!("minkowski() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
        }
    }
    fn indent(&self) -> String {
        self.scad().replace("\n", "\n  ")
    }

}

#[cfg(test)]
mod test {
    use super::*;

    const C5: D2 = D2::Circle(X(5.));
    const S9: D2 = D2::Square(X(9.));
    lazy_static!{ static ref C7: D2 = D2::circle(7); }
    lazy_static!{ static ref C8: D2 = D2::circle(8.0); }

    #[test]
    fn test_circle() {
        assert_eq!(C5.scad(), "circle(r = 5);");
    }

    #[test]
    fn test_cylinder() {
        assert_eq!(D3::cylinder(10.0, 5.0).scad(), "cylinder(h = 10, r = 5);");
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
    fn test_color() {
        assert_eq!(D2::circle(7).add(D2::square(9)).color(Color::Red).scad(),
        "color(\"red\") {\n  union() {\n    circle(r = 7);\n    square(size = 9);\n  }\n}"
        );
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
    fn test_add_map() {
        assert_eq!(S9.iter_rotate(X(20.), 4).union().add_map(|x| x.mirror(XY(1., 0.))).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  mirror(v = [1, 0]) {\n    union() {\n      rotate(0) {\n        square(size = 9);\n      }\n      rotate(20) {\n        square(size = 9);\n      }\n      rotate(40) {\n        square(size = 9);\n      }\n      rotate(60) {\n        square(size = 9);\n      }\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_union_union() {
        assert_eq!(S9.iter_rotate(X(20.), 4).union().add(C5).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_iter_hull() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).hull()),
            "hull() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_minkowski() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).minkowski()),
            "minkowski() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
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
    fn test_linear_extrude() {
        assert_eq!(format!("{}", S9.iter_rotate(X(20.), 4).intersection().linear_extrude(X(10.))),
            "linear_extrude(height = 10) {\n  intersection() {\n    rotate(0) {\n      square(size = 9);\n    }\n    rotate(20) {\n      square(size = 9);\n    }\n    rotate(40) {\n      square(size = 9);\n    }\n    rotate(60) {\n      square(size = 9);\n    }\n  }\n}"
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
