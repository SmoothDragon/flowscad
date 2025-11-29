use ndarray::{Array1, concatenate, Axis, s};
use num_complex::Complex32;
use std::f32::consts::PI;

use crate::Face;
pub use crate::D2Trait;

fn expi(theta: f32) -> Complex32 {
    Complex32::new(theta.cos(), theta.sin())
}

pub fn hypocycloid(k: usize, edges: usize) -> Face {
    let theta = Array1::linspace(0.0, 2.0 * PI, edges+1).slice(ndarray::s![..-1]).to_owned();
    let k: f32 = (k-1) as f32;
    Face(theta.map(|rads| k * expi(*rads) + expi(k * *rads).conj())
        // .slice(ndarray::s![..-1])
        // .to_owned()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hypocycloid() {
        let hypo4: Face = hypocycloid(4, 4).truncated();
        assert_eq!(hypo4.scad(), "polygon(points = [ [4, 0], [0, 4], [-4, 0], [0, -4] ]);");
        let hypo6: Face = hypocycloid(6, 6).truncated();
        assert_eq!(hypo6.scad(), 
            "polygon(points = [ [6, 0], [3, 5.19615], [-3, 5.19615], [-6, 0], [-3, -5.19615], [3, -5.19615] ]);");

    }
}
