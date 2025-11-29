use std::cmp::Ordering;
use std::fmt::Write;
use std::ops::Deref;
use itertools::Itertools;

use ndarray::Array1;
// use ndarray::{concatenate, Axis, s};
use num_complex::Complex32 as C32;

pub use crate::Deg;
pub use crate::Rad;
pub use crate::D2Trait;
use crate::*;

fn exp_i(theta: f32) -> C32 {
    C32::new(theta.cos(), theta.sin())
}

impl From<Vec<C32>> for Face {
    fn from(vector: Vec<C32>) -> Self {
        let array: Array1<C32> = Array1::from_vec(vector);
        Self(array)
    }
}

impl From<&Face> for D2 {
    fn from(face: &Face) -> Self {
        D2::polygon(face.0.iter().map(|xy| XY(xy.re, xy.im)).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face(pub Array1<C32>);

impl Deref for Face {
    type Target = Array1<C32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl D2Trait for Face {
    fn scad(&self) -> String {
        format!("polygon(points = [ {} ]);",
            self.0.iter().map(|xy| format!("[{}, {}]", xy.re, xy.im)).collect::<Vec<_>>().join(", "))
    }

    /// Convert points (x, y) to a single SVG path using Catmull-Rom -> Cubic Bezier
    fn svg(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let mut d = String::new();
        write!(&mut d, "<path d=\"");
        // Move to first point
        write!(&mut d, "M {} {}", self.0[0].re, self.0[0].im).unwrap();

        // For each segment, compute control points
        let n = self.0.len();
        for i in 0..n {
            // p0, p1, p2, p3 for Catmull-Rom
            // let p0 = if i == 0 { self.0[0] } else { self.0[i - 1] };
            let p0 = self.0[(i + n - 1) % n];
            let p1 = self.0[i];
            let p2 = self.0[(i + 1) % n];
            let p3 = self.0[(i + 2) % n];
            // let p3 = if i + 2 >= self.0.len() { self.0[self.0.len() - 1] } else { self.0[i + 2] };

            // Catmull-Rom to cubic Bézier formula
            let c1 = p1 + (p2 - p0) / 6.0;
            let c2 = p2 - (p3 - p1) / 6.0;
            write!(&mut d, " C {} {}, {} {}, {} {}", c1.re, -c1.im, c2.re, -c2.im, p2.re, -p2.im).unwrap();
        }
        write!(&mut d, r#"" stroke="black" fill="none" stroke-width="1"/>"#);

        d
    }


    fn rotate<T: Into<Rad>>(&mut self, theta: T) {
        self.0 *= exp_i(theta.into().0);
    }

    fn scale(&mut self, factor: f32) {
        self.0 *= C32::new(factor, 0.);
    }

   fn translate(&mut self, xy: C32) {
        self.0 += xy;
    }

   fn bbox(&self) -> (C32, C32) {
        let (x_min, y_min, x_max, y_max) = self.0.iter().fold( 
            (f32::INFINITY, f32::INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            |(x_min, y_min, x_max, y_max), &z| (x_min.min(z.re), y_min.min(z.im), x_max.max(z.re), y_max.max(z.im)) );
        (C32::new(x_min, y_min), C32::new(x_max, y_max))
    }

   fn xy(&mut self) {
        let (x_min, y_min) = self.0.iter().fold( (f32::INFINITY, f32::INFINITY),
            |(x_min, y_min), &z| (x_min.min(z.re), y_min.min(z.im)) );
        self.translate(C32::new(-x_min, -y_min));
    }
}

// truncates to the closest millionth
fn truncated(xx: f32) -> f32 {
    ((xx * 100000.).round() as i32) as f32 / 100000.
}

impl Face {
    fn rotate<T: Into<Rad>>(&mut self, theta: T) {
        self.0 *= exp_i(theta.into().0);
    }

    fn translate(&mut self, xy: C32) {
        self.0 += xy;
    }

    pub fn truncated(&self) -> Self {
        Self(Array1::from_iter(self.0.iter().map(|xy| C32::new(truncated(xy.re), truncated(xy.im)))))
    }

    pub fn sorted(&self) -> Self {
        Self::from(self.0.iter().copied()
            .sorted_by(|a, b| 
                a.re.partial_cmp(&b.re).unwrap_or(Ordering::Equal)
                .then(a.im.partial_cmp(&b.im).unwrap_or(Ordering::Equal))
            )
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: this isn't right
    #[test]
    fn test_deref() {
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        let square2 = Face::from(vec![
            C32::new(2.,0.), 
            C32::new(0.,2.), 
            C32::new(-2.,0.), 
            C32::new(0.,-2.),
        ]);
        assert_eq!(square2, Face(square.0 * C32::new(2.0, 0.)));
    }

    #[test]
    fn test_rotated() {
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        // Testing equality is hard
        assert_eq!(square.sorted(), square.rotated(Deg(90.)).truncated().sorted());
    }

    #[test]
    fn test_bbox() {
        let square = Face::from(vec![
            C32::new(0.,0.), 
            C32::new(1.,0.), 
            C32::new(1.,1.), 
            C32::new(0.,1.), 
        ]);
        assert_eq!(square.bbox(), (C32::new(0.,0.), C32::new(1.,1.)) );
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        assert_eq!(square.bbox(), (C32::new(-1.,-1.), C32::new(1.,1.)) );
    }

    #[test]
    fn test_xyed() {
        let square = Face::from(vec![
            C32::new(0.,0.), 
            C32::new(1.,0.), 
            C32::new(1.,1.), 
            C32::new(0.,1.), 
        ]);
        assert_eq!(square, square.xyed());
    }

    #[test]
    fn test_d2_from() {
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        assert_eq!(D2::from(&square).scad(), "polygon(points = [ [1, 0], [0, 1], [-1, 0], [0, -1] ]);");
    }

    #[test]
    fn test_scad() {
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        assert_eq!(square.scad(), "polygon(points = [ [1, 0], [0, 1], [-1, 0], [0, -1] ]);");
        let square = Face::from(vec![
            C32::new(0.,0.), 
            C32::new(1.,0.), 
            C32::new(1.,1.), 
            C32::new(0.,1.), 
        ]);
        assert_eq!(square.scad(), "polygon(points = [ [0, 0], [1, 0], [1, 1], [0, 1] ]);");

    }

    #[test]
    fn test_svg() {
        let square = Face::from(vec![
            C32::new(1.,0.), 
            C32::new(0.,1.), 
            C32::new(-1.,0.), 
            C32::new(0.,-1.),
        ]);
        println!("{}", square.clone().svg());
        println!("{}", (-1 % 10));
        assert_eq!(square.svg(), "<path d=\"M 1 0 C 1 -0.33333334, 0.33333334 -1, 0 -1 C -0.33333334 -1, -1 -0.33333334, -1 -0 C -1 0.33333334, -0.33333334 1, 0 1 C 0.33333334 1, 1 0.33333334, 1 -0\" stroke=\"black\" fill=\"none\" stroke-width=\"1\"/>");
    }
}
