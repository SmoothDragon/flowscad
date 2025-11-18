use core::ops::*;
use core::cmp::*;
use std::f32::consts::PI;

use derive_more::*;

use num_complex::Complex32 as C32;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, Sub, Neg)]
pub struct Deg(pub f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, Sub, Neg)]
pub struct Rad(pub f32);

impl From<Deg> for Rad {
    fn from(degree: Deg) -> Self {
        Self(degree.0 * PI / 180.0)
    }
}

impl From<Rad> for Deg {
    fn from(radian: Rad) -> Self {
        Self(radian.0 * 180.0 / PI)
    }
}


pub trait D2Trait: Clone {
    fn scad(&self) -> String;
    fn svg(&self) -> String;
    fn rotate<T: Into<Rad>>(&mut self, theta: T);
    fn scale(&mut self, factor: f32);
    fn translate(&mut self, xy: C32);
    fn xy(&mut self);  // Align in upper right quadrant
    // fn xyed(&self) -> Self;  // Align in upper right quadrant
    // fn bounding_box(&self) -> (C32, C32);  // lower left and upper right corners of bounding box
    // fn center(&mut self);  // Center the object
    fn rotated<T: Into<Rad>>(&self, theta: T) -> Self {
        let mut shape = self.clone();
        shape.rotate(theta);
        shape
    }

    fn scaled(&self, factor: f32) -> Self {
        let mut shape = self.clone();
        shape.scale(factor);
        shape
    }

    fn translated(&self, xy: C32) -> Self {
        let mut shape = self.clone();
        shape.translate(xy);
        shape
    }

    fn xyed(&self) -> Self {
        let mut shape = self.clone();
        shape.xy();
        shape
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deg_from_rad() {
        assert_eq!(Deg::from(Rad(PI)), Deg(180.));
    }

    #[test]
    fn test_rad_from_deg() {
        assert_eq!(Rad::from(Deg(180.)), Rad(PI));
    }

}
