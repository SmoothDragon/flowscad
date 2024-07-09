
use lazy_static::lazy_static;
use crate::*;
use crate::common::*;

// use common::ColorEnum;

impl<T: Iterator<Item=D3>> DIterator<D3> for T {
    fn hull(self: Self) -> D3 {
        D3::Hull(Box::new(self.collect::<Vec<D3>>()))
        // D2::Join("hull", Box::new(self.collect::<Vec<D2>>()))
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



// #[derive(Clone, Debug)]
// pub struct XYZ(pub f64, pub f64, pub f64);

#[derive(Clone, Debug)]
pub enum D3 {
    Cube(Real),
    Cuboid(Real3),
    Color(ColorEnum, Box<D3>),
    Cylinder(Real, Real),
    Sphere(Real),
    Translate(Real3, Box<D3>),
    Scale(Real, Box<D3>),
    Scale3(Real3, Box<D3>),
    Rotate(Real3, Box<D3>),
    LinearExtrude(Real, Box<D2>),
    RotateExtrude(Real, Box<D2>),
    Hull(Box<Vec<D3>>),
    Intersection(Box<Vec<D3>>),
    Union(Box<Vec<D3>>),
    Minkowski(Box<Vec<D3>>),
    Difference(Box<D3>, Box<D3>),
    Join(&'static str, Box<Vec<D3>>),
    // TODO: Join(&'static str, Box<Vec<D3>>),
}


pub fn indent_d3(shape: &D3) -> String {
    format!("{}", shape).replace("\n", "\n  ")
}

impl std::fmt::Display for D3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.scad())
    }
}

impl std::iter::Sum for D3 {
    fn sum<I>(iter: I) -> Self
      where 
        I: Iterator<Item = Self>
    {
        D3::Union(Box::new(iter.collect::<Vec<Self>>()))
        // D3::Join("union", Box::new(iter.collect::<Vec<Self>>()))
    }
}

impl std::iter::Product for D3 {
    fn product<I>(iter: I) -> Self
      where 
        I: Iterator<Item = Self>
    {
        D3::Intersection(Box::new(iter.collect::<Vec<Self>>()))
    }
}

// trait SCAD {
    // fn scad(&self) -> String;
    // fn indent(&self) -> String;
// }

impl SCAD for D3 {
    fn scad(&self) -> String {
        match &self {
            D3::LinearExtrude(Real(h), shape) => format!("linear_extrude(height = {}) {{\n  {}\n}}", h, indent(shape)),
            D3::RotateExtrude(Real(angle), shape) => format!("rotate_extrude(angle = {}) {{\n  {}\n}}", angle, indent(shape)),
            D3::Cube(size) => format!("cube(size = {});", size),
            D3::Cuboid(Real3(xyz)) => format!("cube(size = [{}, {}, {}]);", xyz.x, xyz.y, xyz.z),
            D3::Sphere(radius) => format!("sphere(r = {});", radius),
            D3::Cylinder(h, r) => format!("cylinder(h = {}, r = {});", h, r),
            D3::Color(color, shape) => format!("color({}) {{\n  {}\n}}", 
                match color {
                    ColorEnum::Blue => "\"blue\"",
                    ColorEnum::Green => "\"green\"",
                    ColorEnum::Red => "\"red\"",
                }
                , shape.indent()),
            D3::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s, shape.indent()),
            D3::Scale3(Real3(v), shape) => format!("scale(v = [{}, {}, {}]) {{\n  {}\n}}", v.x, v.y, v.z, shape.indent()),
            D3::Union(v) => format!( "union() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Hull(v) => format!("hull() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Minkowski(v) => format!("minkowski() {{\n  {}\n}}",
                v.iter().map(|x| format!("{}", indent_d3(x))).collect::<Vec<_>>().join("\n  ")),
            D3::Translate(Real3(xyz), shape) => format!("translate(v = [{}, {}, {}]) {{\n  {}\n}}", xyz.x, xyz.y, xyz.z, shape.indent()),
            D3::Rotate(Real3(xyz), shape) => format!("rotate([{}, {}, {}]) {{\n  {}\n}}", xyz.x, xyz.y, xyz.z, shape.indent()),
            D3::Difference(shape1, shape2) => format!("difference() {{\n  {}\n  {}\n}}", indent_d3(shape1), indent_d3(shape2)),
            D3::Join(name, v) => format!("{}() {{\n  {}\n}}", &name,
                v.iter().map(|x| format!("{}", x.indent())).collect::<Vec<_>>().join("\n  ")),
        }
    }
    fn indent(&self) -> String {
        self.scad().replace("\n", "\n  ")
    }

}

impl D3 {
    /// Create a cube with side length `s` with lower left corner at the origin.
    pub fn cube<T: Into<Real>>(side: T) -> D3 {
        D3::Cube(side.into())
    }

    /// Create a rectangular cuboid with side lengths `x,y,z` with lower left corner at the origin.
    pub fn cuboid(xyz: Real3) -> D3 {
        D3::Cuboid(xyz)
    }

    /// Create a sphere with `radius` centered at the origin.
    pub fn sphere<T: Into<Real>>(radius: T) -> D3 {
        D3::Sphere(radius.into())
    }

    /// Subtract `self` from a cube centered at the origin with edge length `l_edge`.
    // pub fn invert<T: Into<Real>>(self, l_edge: T) -> D3 { TODO
    pub fn invert(self, l_edge: f64) -> D3 {
        let shift = -l_edge/2.0;
        D3::cube(l_edge)
            .translate(v3(shift,shift,shift)) 
            // .translate(-0.5*v3(l_edge,l_edge,l_edge)) 
            - self
    }

    /// Create a spheroid with radii, `r1, r2, r3` centered at the origin.
    pub fn spheroid(radii: Real3) -> D3 {
        D3::Sphere(Real(1.0)).scale3(radii)
    }

    pub fn color(self, color_name: ColorEnum) -> D3 {
        D3::Color(color_name, Box::new(self))
    }

    /// Scale size by the factor `s`.
    pub fn scale<T: Into<Real>>(self, scale_factor: T) -> D3 {
        D3::Scale(scale_factor.into(), Box::new(self.clone()))
    }

    /// Scale in `x` and `y` directions.
    pub fn scale3(self, xyz: Real3) -> D3 {
        D3::Scale3(xyz, Box::new(self))
    }

    pub fn translate(self, xyz: Real3) -> D3 {
        D3::Translate(xyz, Box::new(self.clone()))
    }


    pub fn iter_translate<'a>(&'a self, xyz: Real3, n: u32) -> impl Iterator<Item = D3> + 'a {
        (0..n).map(move |ii| self.clone().translate(v3(xyz.0.x * ii as f32, xyz.0.y * ii as f32, xyz.0.z * ii as f32)))
    }

    // pub fn iter_translate2<'a, X: Into<Real> + 'a, Y: Into<Real> + 'a, Z: Into<Real> + 'a>(&'a self, x: X, y: Y, z: Z, n: u32) 
        // -> impl Iterator<Item = D3> + 'a {
        // (0..n).map(move |ii| self.clone().translate(x.clone().into(), y.clone().into(), z.clone().into()))
    // }

    pub fn rotate(self, xyz: Real3) -> D3 {
        D3::Rotate(xyz, Box::new(self.clone()))
    }


    /// Create a cylinder of height `h` and radius `r` centered above the XY plane.
    pub fn cylinder<H: Into<Real>, R: Into<Real>>(h: H, r:R) -> D3 {
        D3::Cylinder(h.into(), r.into())
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

    pub fn minkowski(self, other: D3) -> D3 {
        D3::Minkowski(Box::new(vec![self, other]))
        /*
        match self { // Combine Minkowski sums if possible
            D2::Join("minkowski", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("minkowski", vec)
                },
            _ => D2::Join("minkowski", Box::new(vec![self, other])),
        }
        */
    }

    pub fn add_map<F>(self, f: F) -> D3 where F: Fn(D3) -> D3 {
        self.clone().add(f(self))
    }



    pub fn iter_rotate<'a>(&'a self, theta: Real3, n: u32) -> impl Iterator<Item = D3> + 'a {
        (0..n).map(move |ii| self.clone().rotate(v3(theta.0.x * ii as f32, theta.0.y * ii as f32, theta.0.z * ii as f32)))
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

    pub fn beveled_box<T: Into<Real>>(xyz: Real3, bevel_in: T) -> D3 {
        let x = xyz.0.x; 
        let y = xyz.0.y;
        let z = xyz.0.z;
        let bevel = bevel_in.into();
        D3::Hull(Box::new(vec![
            D3::cuboid(v3(x,y-bevel*2.,z-bevel*2.)).translate(v3(0.,bevel,bevel)),
            D3::cuboid(v3(x-bevel*2.,y-bevel*2.,z)).translate(v3(bevel,bevel,0.)),
            D3::cuboid(v3(x-bevel*2.,y,z-bevel*2.)).translate(v3(bevel,0.,bevel)),
            ]))
    }

    pub fn truncated_octahedron(l_edge: f64) -> D3 {
        //* Create a truncated ocatahedron with edge length `l_edge` centered at the origin
        let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
        D3::Hull(Box::new(vec![
            D3::cuboid(v3(l_edge, l_edge, 2.0*r_square))
                .translate(v3(-l_edge/2.0, -l_edge/2.0, -r_square))
                .rotate(v3(0., 0., 45.)),
            D3::cuboid(v3(l_edge, 2.*r_square, l_edge))
                .translate(v3(-l_edge/2.0, -r_square, -l_edge/2.0))
                .rotate(v3(0., 45., 0.)),
            D3::cuboid(v3(2.*r_square, l_edge, l_edge))
                .translate(v3(-r_square, -l_edge/2.0, -l_edge/2.0))
                .rotate(v3(45, 0, 0)),
            ]))
    }

    /*
    pub fn truncated_octahedron(r: f64) -> D3 {
        /// Create a truncated ocatahedron with edge length `l_edge` centered at the origin
        /// Create a truncated ocatahedron centered at the origin.
        /// `r_square` equals distance from origin to center of square face.
        // let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
        let r_square = r; // r.into() TODO
        let l_edge = 2.0_f64.powf(0.5) / r_square;  // height of truncated octahedron between square faces
        D3::Hull(Box::new(vec![
            D3::cuboid(v3(l_edge, l_edge, 2.0*r_square))
                .translate(v3(-l_edge/2.0, -l_edge/2.0, -r_square))
                .rotate(v3(0., 0., 45.)),
            D3::cuboid(v3(l_edge, 2.*r_square, l_edge))
                .translate(v3(-l_edge/2.0, -r_square, -l_edge/2.0))
                .rotate(v3(0., 45., 0.)),
            D3::cuboid(v3(2.*r_square, l_edge, l_edge))
                .translate(v3(-r_square, -l_edge/2.0, -l_edge/2.0))
                .rotate(v3(45, 0, 0)),
            ]))
    }
    */

}

impl std::ops::Add<D3> for D3 {
    type Output = D3;

    fn add(self, other: D3) -> D3 {
        self.add(other)
    }
}

impl std::ops::Sub<D3> for D3 {
    type Output = D3;

    fn sub(self, other: D3) -> D3 {
        self.difference(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static!{ static ref S5: D3 = D3::sphere(5); }
    lazy_static!{ static ref C9: D3 = D3::cube(9.0); }

    #[test]
    fn test_sphere() {
        assert_eq!(D3::sphere(5).scad(), "sphere(r = 5);");
    }

    #[test]
    fn test_cylinder() {
        assert_eq!(D3::cylinder(10.0, 5).scad(), "cylinder(h = 10, r = 5);");
    }

    #[test]
    fn test_cube() {
        assert_eq!(D3::cube(9).scad(), "cube(size = 9);");
    }

    #[test]
    fn test_add() {
        assert_eq!(D3::sphere(5).add(D3::cube(9)).scad(),
        "union() {\n  sphere(r = 5);\n  cube(size = 9);\n}");
    }

    #[test]
    fn test_color() {
        assert_eq!(D3::sphere(7_i32).add(D3::cube(9)).color(ColorEnum::Red).scad(),
        "color(\"red\") {\n  union() {\n    sphere(r = 7);\n    cube(size = 9);\n  }\n}"
        );
    }
}
    #[test]
    fn test_iter_translate() {
        assert_eq!(D3::cube(3).iter_translate(v3(1.,2.,3.),4).union().scad(),
            "union() {\n  translate(v = [0, 0, 0]) {\n    cube(size = 3);\n  }\n  translate(v = [1, 2, 3]) {\n    cube(size = 3);\n  }\n  translate(v = [2, 4, 6]) {\n    cube(size = 3);\n  }\n  translate(v = [3, 6, 9]) {\n    cube(size = 3);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate() {
        assert_eq!(D3::cube(3).iter_rotate(v3(10,20,30), 4).sum::<D3>().scad(),
            "union() {\n  rotate([0, 0, 0]) {\n    cube(size = 3);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 3);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 3);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 3);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(D3::cube(3).iter_rotate(v3(10,20,30), 4).product::<D3>().scad(),
            "intersection() {\n  rotate([0, 0, 0]) {\n    cube(size = 3);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 3);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 3);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 3);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(D3::cube(3).iter_rotate(v3(10,20,30), 4).union().scad(),
            "union() {\n  rotate([0, 0, 0]) {\n    cube(size = 3);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 3);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 3);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 3);\n  }\n}"
        );
    }

    /*
    #[test]
    fn test_add_map() {
        assert_eq!(S9.iter_rotate(20, 4).union().add_map(|x| x.mirror(v2(1., 0.))).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  mirror(v = [1, 0]) {\n    union() {\n      rotate(0) {\n        square(size = 9);\n      }\n      rotate(20) {\n        square(size = 9);\n      }\n      rotate(40) {\n        square(size = 9);\n      }\n      rotate(60) {\n        square(size = 9);\n      }\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_union_union() {
        assert_eq!(S9.iter_rotate(20, 4).union().add(D2::circle(5)).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_d2_add_op() {
        assert_eq!((D2::square(9) + D2::circle(5)).scad(),
            "union() {\n  square(size = 9);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_d2_sub_op() {
        assert_eq!((D2::square(9) - D2::circle(5)).scad(),
            "difference() {\n  square(size = 9);\n  circle(r = 5);\n}"
        );
    }

    #[test]
    fn test_d3_add_op() {
        assert_eq!((D3::cube(9) + D3::sphere(5)).scad(),
            "union() {\n  cube(size = 9);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_d3_sub_op() {
        assert_eq!((D3::cube(9) - D3::spheroid(v3(5,4,3))).scad(),
            "difference() {\n  cube(size = 9);\n  scale(v = [5, 4, 3]) {\n    sphere(r = 1);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_hull() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).hull()),
            "hull() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_minkowski() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).minkowski()),
            "minkowski() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_union() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).union()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_intersection() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).intersection()),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_linear_extrude() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).intersection().linear_extrude(10)),
            "linear_extrude(height = 10) {\n  intersection() {\n    rotate(0) {\n      square(size = 9);\n    }\n    rotate(20) {\n      square(size = 9);\n    }\n    rotate(40) {\n      square(size = 9);\n    }\n    rotate(60) {\n      square(size = 9);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_extrude() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).intersection().rotate_extrude(180)),
            "rotate_extrude(angle = 180) {\n  intersection() {\n    rotate(0) {\n      square(size = 9);\n    }\n    rotate(20) {\n      square(size = 9);\n    }\n    rotate(40) {\n      square(size = 9);\n    }\n    rotate(60) {\n      square(size = 9);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate_rotate() {
        assert_eq!(format!("{}", S9.iter_rotate(20, 4).map(move |x| x.rotate(10)).hull()),
            "hull() {\n  rotate(10) {\n    square(size = 9);\n  }\n  rotate(30) {\n    square(size = 9);\n  }\n  rotate(50) {\n    square(size = 9);\n  }\n  rotate(70) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_translate_translate() {
        assert_eq!(C5.iter_translate(v2(1.,2.),4).map(move |x| x.translate(v2(-1., -1.))).union().scad(),
            "union() {\n  translate(v = [-1, -1]) {\n    circle(r = 5);\n  }\n  translate(v = [0, 1]) {\n    circle(r = 5);\n  }\n  translate(v = [1, 3]) {\n    circle(r = 5);\n  }\n  translate(v = [2, 5]) {\n    circle(r = 5);\n  }\n}"
        );
    }


    #[test]
    fn test_triangle() {
        assert_eq!(D2::triangle(v2(0.,0.), v2(1., 0.), v2(0., 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

    #[test]
    fn test_polygon() {
        assert_eq!(D2::polygon(vec![v2(0.,0.), v2(1., 0.), v2(0., 1.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

}
*/
