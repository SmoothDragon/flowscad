//! Create OpenSCAD files using Rust.


use crate::*;

pub const MAX2: f32 = 1000.;

impl<T: Iterator<Item=D2>> DIterator<D2> for T {
    fn hull(self) -> D2 {
        // D2::Hull(Box::new(self.collect::<Vec<D2>>()))
        D2::Join("hull", Box::new(self.collect::<Vec<D2>>()))
    }

    fn union(self) -> D2 {
        // D2::Union(Box::new(self.collect::<Vec<D2>>()))
        D2::Join("union", Box::new(self.collect::<Vec<D2>>()))
    }

    fn intersection(self) -> D2 {
        // D2::Intersection(Box::new(self.collect::<Vec<D2>>()))
        D2::Join("intersection", Box::new(self.collect::<Vec<D2>>()))
    }

    fn minkowski(self) -> D2 {
        // D2::Minkowski(Box::new(self.collect::<Vec<D2>>()))
        D2::Join("minkowski", Box::new(self.collect::<Vec<D2>>()))
    }
}

impl std::ops::Add<D2> for D2 {
    type Output = D2;

    fn add(self, other: D2) -> D2 {
        self.add(other)
    }
}

impl std::ops::Sub<D2> for D2 {
    type Output = D2;

    fn sub(self, other: D2) -> D2 {
        self.difference(other)
    }
}

#[derive(Clone, Debug)]
pub enum Aim {
    N, S, E, W,
    U, D,
    // L, R,
    // Angle(X),
}


#[derive(Clone, Debug)]
pub enum D2 {
    Circle(X),
    Square(X),
    Rectangle(XY),
    RoundedRectangle(XY, X),
    Polygon(Box<Vec<XY>>),
    Color(ColorEnum, Box<D2>),
    Rotate(X, Box<D2>),
    Scale(X, Box<D2>),
    Scale2(XY, Box<D2>),
    Translate(XY, Box<D2>),
    Mirror(XY, Box<D2>),
    OffsetRadius(X, Box<D2>),
    OffsetDelta(X, Box<D2>),
    OffsetChamfer(X, Box<D2>),
    // Hull(Box<Vec<D2>>),
    // Intersection(Box<Vec<D2>>),
    // Union(Box<Vec<D2>>),
    // Minkowski(Box<Vec<D2>>),
    Join(&'static str, Box<Vec<D2>>),
    Svg(String),
    Difference(Box<D2>, Box<D2>),
}

pub fn indent(shape: &D2) -> String {
    format!("{}", shape).replace('\n', "\n  ")
}
impl D2 {
    /// Create a circle of `diameter` centered at the origin.
    pub fn circle_d<T: Into<X>>(diameter: T) -> D2 {
        D2::Circle(diameter.into())
    }

    /// Create a circle of `radius` centered at the origin.
    pub fn circle_r<IX: Into<X>>(radius: IX) -> D2 {
        D2::Circle(2*radius.into())
    }

    /// Create a square with side length `side` with lower left corner at the origin.
    pub fn square<T: Into<X>>(side: T) -> D2 {
        D2::Square(side.into())
    }

    /// Create a rectangle with lower left corner at the origin.
    pub fn rectangle<IXY: Into<XY>>(xy: IXY) -> D2 {
        D2::Rectangle(xy.into())
    }

    /// Create a rounded rectangle with lower left corner at the origin.
    pub fn rounded_rectangle<IX: Into<X>, IXY: Into<XY>>(ixy: IXY, into_radius: IX) -> D2 {
        D2::RoundedRectangle(ixy.into(), into_radius.into())
    }

    pub fn sector<IR: Into<X>, IT: Into<X>>(i_r: IR, i_theta: IT) -> D2 {
        let r = i_r.into();
        let theta = i_theta.into();
        if r < X(180.0) {
            D2::half_plane(Aim::S)
                .rotate(theta)
                .intersection(D2::half_plane(Aim::N))
                .intersection(D2::circle_r(r))
        } else {
            D2::half_plane(Aim::S)
                .rotate(theta)
                .add(D2::half_plane(Aim::N))
                .intersection(D2::circle_r(r))
        }
    }

    pub fn from_svg(filename: &str) -> D2 {
        D2::Svg(filename.to_string())
    }

    /// Center an object, if we know how
    pub fn center(self) -> D2 {
        match self {
            D2::Square(s) => self.translate(-v2(s,s)/2),
            D2::Rectangle(XY(x,y)) => self.translate(-v2(x,y)/2),
            D2::RoundedRectangle(XY(x,y), X(_)) => self.translate(-v2(x,y)/2),
            _ => self,
        }
    }

    /// Round an object, if we know how
    pub fn round<IX: Into<X>>(self, ir: IX) -> D2 {
        let r = ir.into();
        match self {
            D2::Rectangle(XY(x,y)) =>
                D2::circle_r(r)
                    .add_map(move |shape| shape.translate_x(x - 2 * r))
                    .add_map(move |shape| shape.translate_y(y - 2 * r))
                    .hull()
                    .translate(v2(r,r))
                ,
            _ => self,
        }
    }

    /// Offset shape by circle of radius `r`.
    /// If `r` is positive, equivalent to Minkowski sum with circle of radius `r`.
    pub fn offset_radius<IX: Into<X>>(self, ix: IX) -> D2 {
        D2::OffsetRadius(ix.into(), Box::new(self))
    }

    pub fn offset_delta<IX: Into<X>>(self, ix: IX) -> D2 {
        D2::OffsetDelta(ix.into(), Box::new(self))
    }

    pub fn offset_chamfer<IX: Into<X>>(self, ix: IX) -> D2 {
        D2::OffsetChamfer(ix.into(), Box::new(self))
    }


    pub fn add(self, other: D2) -> D2 {
        match self { // Combine Unions if possible
            D2::Join("union", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("union", vec)
                },
            _ => D2::Join("union", Box::new(vec![self, other])),
        }
    }

    pub fn difference(self, other: D2) -> D2 {
        D2::Difference(Box::new(self), Box::new(other))
    }

    pub fn sub(self, other: D2) -> D2 {
        D2::Difference(Box::new(self), Box::new(other))
    }

    pub fn and(self, other: D2) -> D2 {
        D2::intersection(self, other)
    }

    pub fn half_plane(aim: Aim) -> D2 {
        match aim {
            Aim::N => D2::square(MAX2).translate(v2(-MAX2/2., 0.)),
            Aim::S => D2::square(MAX2).translate(v2(-MAX2/2., -MAX2)),
            Aim::E => D2::square(MAX2).translate(v2(0., -MAX2/2.)),
            Aim::W => D2::square(MAX2).translate(v2(-MAX2, -MAX2/2.)),
            Aim::U => D2::square(MAX2).translate(v2(-MAX2/2., 0.)),
            Aim::D => D2::square(MAX2).translate(v2(-MAX2/2., -MAX2)),
            // Aim::Angle(theta) => D2::Square(StrictlyPositiveFinite(MAX2)).translate(XY(0., -MAX2/2.)).rotate(*theta),
            }
    }

    pub fn add_map<F>(self, f: F) -> D2 where F: Fn(D2) -> D2 {
        self.clone().add(f(self))
    }

    pub fn hull(self) -> D2 {
        match self { // Combine Unions if possible
            D2::Join("union", vec) => D2::Join("hull", vec),
            _ => D2::Join("hull", Box::new(vec![self])),
        }
    }

    pub fn intersection(self, other: D2) -> D2 {
        match self { // Combine intersections if possible
            D2::Join("intersection", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("intersection", vec)
                },
            _ => D2::Join("intersection", Box::new(vec![self, other])),
        }
    }

    pub fn minkowski(self, other: D2) -> D2 {
        match self { // Combine Minkowski sums if possible
            D2::Join("minkowski", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("minkowski", vec)
                },
            _ => D2::Join("minkowski", Box::new(vec![self, other])),
        }
    }

    pub fn triangle(xy0: XY, xy1: XY, xy2: XY) -> D2 {
        D2::Polygon(Box::new(vec![xy0, xy1, xy2]))
    }

    pub fn triangle2<P1: Into<XY>, P2: Into<XY>, P3: Into<XY>>(xy0: P1, xy1: P2, xy2: P3) -> D2 {
        D2::Polygon(Box::new(vec![xy0.into(), xy1.into(), xy2.into()]))
    }

    pub fn polygon(points: Vec<XY>) -> D2 {
        D2::Polygon(Box::new(points))
    }

    /// Create a polygon from convex hull of vertices.
    pub fn convex_hull<T: Into<XY>, I: IntoIterator<Item=T>>(points: I) -> D2 {
        let vertices = points.into_iter().map(|w| {
            let v = w.into(); [v.0 as f64, v.1 as f64]
        }).collect::<Vec<[f64; 2]>>();
        let points = convex_hull_2d(vertices);
        D2::Polygon(
            Box::new(
                points.into_iter()
                .map(Into::<XY>::into)
                .collect::<Vec<XY>>()
                )
            )
    }

    pub fn regular_polygon<IX: Into<X>>(num_sides: u32, i_radius: IX) -> D2 {
        let r: XY = v2(i_radius.into(), 0);
        let theta: X = 360 / Into::<X>::into(num_sides);
        D2::convex_hull(
            (0..num_sides)
            .map(|ii| Into::<[f32; 2]>::into(r.rotate_deg(ii * theta)))
            .collect::<Vec<[f32; 2]>>()
            )
    }


    pub fn rounded_regular_polygon<IX: Into<X>, IR: Into<X>>(num_sides: u32, radius: IX, r_corner: IR) -> D2 {
        let r: X = radius.into();
        let r_c: X = r_corner.into();
        let n: X = num_sides.into();
        D2::circle_r(r_c)
            .translate_x(r - r_c)
            .iter_rotate(360/n, num_sides)
            .hull()
    }

    pub fn polygon2<IXY: Into<XY> + Clone>(points: Vec<IXY>) -> D2 {
        D2::Polygon(Box::new(points.iter().map(|xy| xy.clone().into()).collect()))
    }

    pub fn translate<IXY: Into<XY>>(&self, xy: IXY) -> D2 {
        // TODO: Is clone needed here?
        match self {
            D2::Translate(v, d2) => D2::Translate(*v+xy.into(), d2.clone()),
            _ => D2::Translate(xy.into(), Box::new(self.clone())),
        }
    }

    pub fn translate_x<IX: Into<X>>(self, x: IX) -> D2 {
        D2::Translate(v2(x,0), Box::new(self))
    }

    pub fn translate_y<IX: Into<X>>(self, y: IX) -> D2 {
        D2::Translate(v2(0,y), Box::new(self))
    }


    pub fn mirror(&self, xy: XY) -> D2 {
        D2::Mirror(xy, Box::new(self.clone()))
    }

    pub fn iter_translate(&self, xy: XY, n: u32) -> impl Iterator<Item = D2> + '_ {
        (0..n).map(move |ii| self.translate(v2(xy.0 * ii as f32, xy.1 * ii as f32)))
    }

    pub fn rotate<IX: Into<X>>(&self, theta: IX) -> D2 {
        match self {
            D2::Rotate(phi, d2) => D2::Rotate(*phi + theta.into(), d2.clone()),
            _ => D2::Rotate(theta.into(), Box::new(self.clone())),
        }
    }

    pub fn iter_rotate<IX: Into<X>>(&self, theta: IX, n: u32) -> impl Iterator<Item = D2> + '_ {
        let angle = theta.into();
        // (0..n).map(move |ii| self.rotate(theta.into() * <u32 as Into<X>>::ii.into()))
        (0..n).map(move |ii| self.rotate(angle * ii as f32))
    }

    pub fn iter_rotate_equal(&self, n: u32) -> impl Iterator<Item = D2> + '_ {
        (0..n).map(move |ii| self.rotate(360./(n as f64) * ii as f64))
    }

    pub fn iter_square_edge<D: Into<X>>(&self, d: D) -> impl Iterator<Item = D2> + '_ {
        let shift = d.into();
        vec![v2(shift, 0.), v2(0., shift), v2(-shift, 0.), v2(0., -shift)]
            .into_iter()
            .map(move |xy| self.translate(xy))
    }

    pub fn translate_vec<IXY: Into<XY> + Clone>(&self, ixy: IXY, n: u32) -> Vec<D2> {
        let xy: XY = ixy.into();
        (0..n).map(move |ii| self.translate(v2(xy.0 * ii as f32, xy.1 * ii as f32))).collect::<Vec<_>>()
    }

    pub fn color(self, color_name: ColorEnum) -> D2 {
        D2::Color(color_name, Box::new(self))
    }

    /// Scale size by the factor `s`.
    pub fn scale<T: Into<X>>(self, scale_factor: T) -> D2 {
        D2::Scale(scale_factor.into(), Box::new(self.clone()))
    }

    /// Scale in `x` and `y` directions.
    pub fn scale2(self, xy: XY) -> D2 {
        D2::Scale2(xy, Box::new(self))
    }

    pub fn linear_extrude<IX: Into<X>>(&self, x: IX) -> D3 {
        D3::LinearExtrude(x.into(), Box::new(self.clone()))
    }

    pub fn rotate_extrude<IX: Into<X>>(&self, x: IX) -> D3 {
        D3::RotateExtrude(x.into(), Box::new(self.clone()))
    }
}

impl std::iter::Sum for D2 {
    fn sum<I>(iter: I) -> Self
      where
        I: Iterator<Item = Self>
    {
        D2::Join("union", Box::new(iter.collect::<Vec<Self>>()))
    }
}

impl std::iter::Product for D2 {
    fn product<I>(iter: I) -> Self
      where
        I: Iterator<Item = Self>
    {
        D2::Join("intersection", Box::new(iter.collect::<Vec<Self>>()))
    }
}

impl std::fmt::Display for D2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.scad())
    }
}
impl SCAD for D2 {
    fn scad(&self) -> String {
        match &self {
            D2::Circle(diameter) => format!("circle(d = {});", diameter),
            D2::Square(size) => format!("square(size = {});", size),
            D2::Rectangle(XY(x,y)) => format!("square(size = [{}, {}]);", x, y),
            D2::RoundedRectangle(XY(x,y), r) => format!("{};",
                D2::circle_r(*r)
                    .add_map(move |shape| shape.translate_x(*x - 2 * *r))
                    .add_map(move |shape| shape.translate_y(*y - 2 * *r))
                    .hull()
                    .translate(v2(*r,*r))
                ),
            D2::Polygon(points) => format!("polygon(points = [ {} ]);",
                points.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ")),
            D2::Color(color, shape) => format!("color({}) {{\n  {}\n}}",
                match color {
                    ColorEnum::Blue => "\"blue\"",
                    ColorEnum::Green => "\"green\"",
                    ColorEnum::Red => "\"red\"",
                }
                , indent(shape)),
            D2::Svg(filename) => format!("import(\"{}\", center=true);", filename),
            D2::Translate(XY(x,y), shape) => format!("translate(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
            D2::Mirror(XY(x,y), shape) => format!("mirror(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
            D2::OffsetRadius(X(x), shape) => format!("offset(r = {}) {{\n  {}\n}}", x, indent(shape)),
            D2::OffsetDelta(X(x), shape) => format!("offset(delta = {}) {{\n  {}\n}}", x, indent(shape)),
            D2::OffsetChamfer(X(x), shape) => format!("offset(delta = {}, chamfer=true) {{\n  {}\n}}", x, indent(shape)),
            D2::Rotate(X(theta), shape) => format!("rotate({}) {{\n  {}\n}}", theta, indent(shape)),
            D2::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s, indent(shape)),
            D2::Scale2(XY(x,y), shape) => format!("scale(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
            // D2::Union(v) => format!( "union() {{\n  {}\n}}",
                // v.iter().map(|x| x.indent()).collect::<Vec<_>>().join("\n  ")),
            // D2::Hull(v) => format!("hull() {{\n  {}\n}}",
                // v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            // D2::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                // v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            // D2::Minkowski(v) => format!("minkowski() {{\n  {}\n}}",
                // v.iter().map(|x| format!("{}", indent(x))).collect::<Vec<_>>().join("\n  ")),
            D2::Join(name, v) => format!("{}() {{\n  {}\n}}", &name,
                v.iter().map(|x| indent(x).to_string()).collect::<Vec<_>>().join("\n  ")),
            D2::Difference(shape1, shape2) => format!("difference() {{\n  {}\n  {}\n}}", indent(shape1), indent(shape2)),
        }
    }
    fn indent(&self) -> String {
        self.scad().replace('\n', "\n  ")
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static!{ static ref C5: D2 = D2::circle_d(5); }
    lazy_static!{ static ref C7: D2 = D2::circle_d(7); }
    lazy_static!{ static ref C8: D2 = D2::circle_d(8.0); }
    lazy_static!{ static ref S9: D2 = D2::square(9.0); }

    #[test]
    fn test_circle() {
        assert_eq!(D2::circle_d(5).scad(), "circle(d = 5);");
        assert_eq!(D2::circle_r(5).scad(), "circle(d = 10);");
    }

    #[test]
    fn test_square() {
        assert_eq!(S9.scad(), "square(size = 9);");
    }

    #[test]
    fn test_square_center() {
        assert_eq!(D2::square(9).center().scad(),
            "translate(v = [-4.5, -4.5]) {\n  square(size = 9);\n}"
            );
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(D2::rectangle( (5,9.0) ).scad(),
          "square(size = [5, 9]);"
          );
    }

    #[test]
    fn test_rectangle_center() {
        assert_eq!(D2::rectangle( (5,9.0) ).center().scad(),
          "translate(v = [-2.5, -4.5]) {\n  square(size = [5, 9]);\n}"
          );
    }

    #[test]
    fn test_rounded_rectangle() {
        assert_eq!(D2::rounded_rectangle( (5,9.0) , 2.).scad(),
            "translate(v = [2, 2]) {\n  hull() {\n    circle(d = 4);\n    translate(v = [1, 0]) {\n      circle(d = 4);\n    }\n    translate(v = [0, 5]) {\n      union() {\n        circle(d = 4);\n        translate(v = [1, 0]) {\n          circle(d = 4);\n        }\n      }\n    }\n  }\n};"
          );
    }

    #[test]
    fn test_rounded_rectangle_center() {
        assert_eq!(D2::rounded_rectangle( (5,9.0), 2 ).center().scad(),
            "translate(v = [-2.5, -4.5]) {\n  translate(v = [2, 2]) {\n    hull() {\n      circle(d = 4);\n      translate(v = [1, 0]) {\n        circle(d = 4);\n      }\n      translate(v = [0, 5]) {\n        union() {\n          circle(d = 4);\n          translate(v = [1, 0]) {\n            circle(d = 4);\n          }\n        }\n      }\n    }\n  };\n}"

        );
    }

    #[test]
    fn test_add() {
        assert_eq!(D2::circle_d(5).add(D2::square(9)).scad(),
        "union() {\n  circle(d = 5);\n  square(size = 9);\n}");
    }

    #[test]
    fn test_color() {
        assert_eq!(D2::circle_d(7_i32).add(D2::square(9)).color(ColorEnum::Red).scad(),
        "color(\"red\") {\n  union() {\n    circle(d = 7);\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_translate() {
        assert_eq!(C5.iter_translate(v2(1.,2.),4).union().scad(),
            "union() {\n  translate(v = [0, 0]) {\n    circle(d = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(d = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(d = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(d = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate() {
        assert_eq!(S9.iter_rotate(20, 4).sum::<D2>().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(S9.iter_rotate(20, 4).product::<D2>().scad(),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(S9.iter_rotate(20, 4).union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_add_map() {
        assert_eq!(S9.iter_rotate(20, 4).union().add_map(|x| x.mirror(v2(1., 0.))).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  mirror(v = [1, 0]) {\n    union() {\n      rotate(0) {\n        square(size = 9);\n      }\n      rotate(20) {\n        square(size = 9);\n      }\n      rotate(40) {\n        square(size = 9);\n      }\n      rotate(60) {\n        square(size = 9);\n      }\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_union_union() {
        assert_eq!(S9.iter_rotate(20, 4).union().add(D2::circle_d(5)).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  circle(d = 5);\n}"
        );
    }

    #[test]
    fn test_d2_add_op() {
        assert_eq!((D2::square(9) + D2::circle_d(5)).scad(),
            "union() {\n  square(size = 9);\n  circle(d = 5);\n}"
        );
    }

    #[test]
    fn test_d2_sub_op() {
        assert_eq!((D2::square(9) - D2::circle_d(5)).scad(),
            "difference() {\n  square(size = 9);\n  circle(d = 5);\n}"
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
            "union() {\n  translate(v = [-1, -1]) {\n    circle(d = 5);\n  }\n  translate(v = [0, 1]) {\n    circle(d = 5);\n  }\n  translate(v = [1, 3]) {\n    circle(d = 5);\n  }\n  translate(v = [2, 5]) {\n    circle(d = 5);\n  }\n}"
        );
    }


    #[test]
    fn test_triangle() {
        assert_eq!(D2::triangle(v2(0.,0.), v2(1., 0.), v2(0., 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

    #[test]
    fn test_triangle2() {
        assert_eq!(D2::triangle2((0.,0.), (1., 0.), (0., 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::triangle2((0,0), v2(1., 0.), (0, 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

    #[test]
    fn test_polygon() {
        assert_eq!(D2::polygon(vec![v2(0.,0.), v2(1., 0.), v2(0., 1.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

    #[test]
    fn test_polygon2() {
        assert_eq!(D2::polygon2(vec![v2(0.,0.), v2(1., 0.), v2(0., 1.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::polygon2(vec![(0.,0.), (1., 0.), (0., 1.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::polygon2(vec![(0,0), (1, 0), (0, 1)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
    }

    #[test]
    fn test_convex_hull() {
        assert_eq!(D2::convex_hull(vec![v2(0.,0.), v2(1., 0.), v2(0., 1.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::convex_hull(vec![v2(0.,0.), v2(0., 1.), v2(1., 0.)]).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::convex_hull(vec![
            v2(0.,0.), v2(0., 1.), v2(1., 0.),
            v2(2.,0.), v2(2., 1.), v2(1, 1),
            v2(0.,2.), v2(2., 2.), v2(1., 0.)
        ]).scad(),
            "polygon(points = [ [0, 0], [2, 0], [2, 2], [0, 2] ]);");
    }

    #[test]
    fn test_regular_polygon() {
        assert_eq!(D2::regular_polygon(4, 10).scad(),
            "polygon(points = [ [-0.0000004371139, 10], [-10, -0.0000008742278], [0.00000011924881, -10], [10, 0] ]);");
    }

    #[test]
    fn test_rounded_regular_polygon() {
        assert_eq!(D2::rounded_regular_polygon(5, 10, 1).scad(),
            "hull() {\n  rotate(0) {\n    translate(v = [9, 0]) {\n      circle(d = 2);\n    }\n  }\n  rotate(72) {\n    translate(v = [9, 0]) {\n      circle(d = 2);\n    }\n  }\n  rotate(144) {\n    translate(v = [9, 0]) {\n      circle(d = 2);\n    }\n  }\n  rotate(216) {\n    translate(v = [9, 0]) {\n      circle(d = 2);\n    }\n  }\n  rotate(288) {\n    translate(v = [9, 0]) {\n      circle(d = 2);\n    }\n  }\n}");
    }

    #[test]
    fn test_sector() {
        assert_eq!(D2::sector(10, 30).scad(),
            "intersection() {\n  rotate(30) {\n    translate(v = [-500, -1000]) {\n      square(size = 1000);\n    }\n  }\n  translate(v = [-500, 0]) {\n    square(size = 1000);\n  }\n  circle(d = 20);\n}");
    }

}
