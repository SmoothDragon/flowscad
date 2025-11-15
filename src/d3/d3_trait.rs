use core::ops::*;
use core::cmp::*;
use std::f32::consts::PI;

use derive_more::*;


/*
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
*/

pub trait D3Trait {
    fn scad(&self) -> String;
    fn svg(&self) -> String;
    // fn rotated<T: Into<Rad>>(&self, theta: (f32, f32, f32)) -> Self;
    // fn translated(&self, xyz: (f32, f32, f32)) -> Self;
    // fn xyzed(&self) -> Self;  // Align in upper right quadratn
    // fn bounding_box(&self) -> (C32, C32);  // lower left and upper right corners of bounding box
    // fn center(&mut self);  // Center the object
}

#[cfg(test)]
mod test {
    use super::*;

}
