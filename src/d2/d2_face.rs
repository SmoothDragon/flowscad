use std::cmp::Ordering;
use itertools::Itertools;

use ndarray::Array1;
// use ndarray::{concatenate, Axis, s};
use num_complex::Complex32 as C32;

pub use crate::Deg;
pub use crate::Rad;
pub use crate::D2Trait;

fn exp_i(theta: f32) -> C32 {
    C32::new(theta.cos(), theta.sin())
}

impl From<Vec<C32>> for Face {
    fn from(vector: Vec<C32>) -> Self {
        let array: Array1<C32> = Array1::from_vec(vector);
        Self(array)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face(pub Array1<C32>);

impl D2Trait for Face {
    fn scad(&self) -> String {
        format!("polygon(points = [ {} ]);",
            self.0.iter().map(|xy| format!("[{}, {}]", xy.re, xy.im)).collect::<Vec<_>>().join(", "))
    }

    fn svg(&self) -> String {
        String::new()
    }

    fn rotated<T: Into<Rad>>(&self, theta: T) -> Self {
        Self(self.0.clone() * exp_i(theta.into().0))
    }

    fn translated(&self, xy: C32) -> Self {
        Self(self.0.clone() + xy)
    }

    fn xyed(&self) -> Self {
        let (x_min, y_min) = self.0.iter().fold( (f32::INFINITY, f32::INFINITY),
            |(x_min, y_min), &z| (x_min.min(z.re), y_min.min(z.im)) );
        self.translated(C32::new(-x_min, -y_min))
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

    #[test]
    fn test_rotate() {
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

}
