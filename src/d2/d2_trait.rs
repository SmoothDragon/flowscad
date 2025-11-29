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

impl Mul<Deg> for f32 {
    type Output = Deg;

    fn mul(self, RHS: Deg) -> Self::Output {
        Deg(self * RHS.0)
    }
}

impl From<Rad> for Deg {
    fn from(radian: Rad) -> Self {
        Self(radian.0 * 180.0 / PI)
    }
}

impl From<Rad> for C32 {
    fn from(radian: Rad) -> Self {
        Self::new(radian.0.cos(), radian.0.sin())
    }
}

impl From<Deg> for C32 {
    fn from(degree: Deg) -> Self {
        let radian: Rad = degree.into();
        Self::new(radian.0.cos(), radian.0.sin())
    }
}


pub trait D2Trait: Clone {
    fn scad(&self) -> String;
    fn svg(&self) -> String;
    fn rotate<T: Into<Rad>>(&mut self, theta: T);
    fn scale(&mut self, factor: f32);
    fn translate(&mut self, xy: C32);
    fn bbox(&self) -> (C32, C32);  // lower left and upper right corners of bounding box

    fn xy(&mut self) {  // Align in upper right quadrant
        let (xy_min, _) = self.bbox();
        self.translate(-xy_min);
    }

    fn xyed(&self) -> Self {
        let mut shape = self.clone();
        shape.xy();
        shape
    }

    fn center(&mut self) {
        let (xy_min, xy_max) = self.bbox();
        self.translate(-(xy_min+xy_max)/2.0);
    }

    fn centered(&self) -> Self {
        let mut shape = self.clone();
        shape.center();
        shape
    }

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

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f32_mul_deg() {
        assert_eq!(2.0*Deg(10.), Deg(20.));
    }

    #[test]
    fn test_deg_from_rad() {
        assert_eq!(Deg::from(Rad(PI)), Deg(180.));
    }

    #[test]
    fn test_rad_from_deg() {
        assert_eq!(Rad::from(Deg(180.)), Rad(PI));
    }

    #[test]
    fn test_c32_from_rad() {
        assert_eq!(C32::from(Rad(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Rad(PI/4.)), C32::new(0.70710677, 0.70710677));
        assert_eq!(C32::from(Rad(PI/2.)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Rad(PI)), C32::new(-1., -8.742278e-8));
    }

    #[test]
    fn test_c32_from_deg() {
        assert_eq!(C32::from(Deg(0.)), C32::new(1., 0.));
        assert_eq!(C32::from(Deg(45.)), C32::new(0.70710677, 0.70710677));
        assert_eq!(C32::from(Deg(90.)), C32::new(-4.371139e-8, 1.0));
        assert_eq!(C32::from(Deg(180.)), C32::new(-1., -8.742278e-8));
    }

}
