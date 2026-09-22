use ndarray::Array1;
use num_complex::Complex32;

use crate::common::PI;

use crate::Face;


fn expi(theta: f32) -> Complex32 {
    Complex32::new(theta.cos(), theta.sin())
}

pub fn circle(r: f32, edges: usize) -> Face {
    let theta = Array1::linspace(0.0, 2.0 * PI, edges+1).slice(ndarray::s![..-1]).to_owned();
    Face(theta.map(|rads| r * expi(*rads)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::D2Trait;

    #[test]
    fn test_circle() {
        let circ: Face = circle(4., 4).truncated();
        assert_eq!(circ.scad(), "polygon(points = [ [4, 0], [0, 4], [-4, 0], [0, -4] ]);");
    }
}
