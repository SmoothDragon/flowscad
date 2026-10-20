use num_complex::Complex32 as C32;

use crate::Rad;

// pub trait Scad: Clone {
    // fn scad(&self) -> String;
// }

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
