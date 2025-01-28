use crate::*;

impl<T: Iterator<Item=D3>> DIterator<D3> for T {
    fn hull(self) -> D3 {
        D3::Hull(Box::new(self.collect::<Vec<D3>>()))
        // D2::Join("hull", Box::new(self.collect::<Vec<D2>>()))
    }

    fn union(self) -> D3 {
        D3::Union(Box::new(self.collect::<Vec<D3>>()))
    }

    fn intersection(self) -> D3 {
        D3::Intersection(Box::new(self.collect::<Vec<D3>>()))
    }

    fn minkowski(self) -> D3 {
        D3::Minkowski(Box::new(self.collect::<Vec<D3>>()))
    }
}


#[derive(Clone, Debug)]
pub enum D3 {
    Cube(X),
    Cuboid(XYZ),
    Color(ColorEnum, Box<D3>),
    Cylinder(X, X),
    Frustum(X, X, X),
    Sphere{radius: X},
    Polyhedron(Box<Vec<XYZ>>, Box<Vec<Vec<u32>>>),
    Translate(XYZ, Box<D3>),
    Scale(X, Box<D3>),
    Scale3(XYZ, Box<D3>),
    Rotate(XYZ, Box<D3>),
    Mirror(XYZ, Box<D3>),
    // LinearExtrude(X, Box<D2>),
    LinearExtrude{height: X, twist: X, slices: u32, center: bool, shape: Box<D2>},
    RotateExtrude(X, Box<D2>),
    Hull(Box<Vec<D3>>),
    Intersection(Box<Vec<D3>>),
    Union(Box<Vec<D3>>),
    Minkowski(Box<Vec<D3>>),
    Difference(Box<D3>, Box<D3>),
    Join(&'static str, Box<Vec<D3>>),
    // TODO: Join(&'static str, Box<Vec<D3>>),
    Render(Box<D3>),
}


pub fn indent_d3(shape: &D3) -> String {
    format!("{}", shape).replace('\n', "\n  ")
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

impl core::ops::AddAssign for D3 {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl core::ops::Add<D3> for D3 {
    type Output = D3;

    fn add(self, other: Self) -> Self {
        match self { // Combine Unions if possible
            D3::Union(vec) => {
                let mut vec = vec;
                vec.push(other);
                D3::Union(vec)
                },
            _ => D3::Union(Box::new(vec![self, other])),
        }
    }
}


impl std::ops::Sub<D3> for D3 {
    type Output = D3;

    fn sub(self, other: D3) -> D3 {
        self.difference(other)
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

impl BitAnd<D3> for D3 {
    type Output = D3;

    fn bitand(self, other: D3) -> D3 {
        match self { // Combine intersections if possible
            D3::Join("intersection", vec) => {
                let mut vec = vec;
                vec.push(other);
                D3::Join("intersection", vec)
                },
            _ => D3::Join("intersection", Box::new(vec![self, other])),
        }
    }
}


impl SCAD for D3 {
    fn scad(&self) -> String {
        match &self {
            D3::LinearExtrude{height, twist, slices, shape, center} => format!("linear_extrude(height = {}, twist = {}, slices = {}, center = {}) {{\n  {}\n}}", height, twist, slices, center, indent(shape)),
            // D3::LinearExtrude{height: height, twist: twist, slices: slices, shape: shape} => format!("linear_extrude(height = {}, twist = {}, slices = {}) {{\n  {}\n}}", height, twist, slices, indent(shape)),
            D3::RotateExtrude(X(angle), shape) => format!("rotate_extrude(angle = {}) {{\n  {}\n}}", angle, indent(shape)),
            D3::Cube(size) => format!("cube(size = {});", size),
            D3::Cuboid(xyz) => format!("cube(size = [{}, {}, {}]);", xyz.0, xyz.1, xyz.2),
            D3::Sphere{radius} => format!("sphere(r = {});", radius),
            D3::Cylinder(h, d) => format!("cylinder(h = {}, d = {});", h, d),
            D3::Frustum(h, d1, d2) => format!("cylinder(h = {}, d1 = {}, d2 = {});", h, d1, d2),
            D3::Polyhedron(points, faces) => format!("polyhedron(points = [{}], faces = {:?});", 
                points.iter().map(|xyz| format!("{}", xyz)).collect::<Vec<_>>().join(", "),
                faces),
            D3::Color(color, shape) => format!("color({}) {{\n  {}\n}}", 
                match color {
                    ColorEnum::Blue => "\"blue\"",
                    ColorEnum::Green => "\"green\"",
                    ColorEnum::Red => "\"red\"",
                }
                , shape.indent()),
            D3::Scale(s, shape) => format!("scale(v = {}) {{\n  {}\n}}", s, shape.indent()),
            D3::Scale3(v, shape) => format!("scale(v = [{}, {}, {}]) {{\n  {}\n}}", v.0, v.1, v.2, shape.indent()),
            D3::Mirror(XYZ(x,y,z), shape) => format!("mirror(v = [{}, {}, {}]) {{\n  {}\n}}", x, y, z, shape.indent()),
            D3::Union(v) => format!( "union() {{\n  {}\n}}",
                v.iter().map(|x| indent_d3(x).to_string()).collect::<Vec<_>>().join("\n  ")),
            D3::Hull(v) => format!("hull() {{\n  {}\n}}",
                v.iter().map(|x| indent_d3(x).to_string()).collect::<Vec<_>>().join("\n  ")),
            D3::Intersection(v) => format!("intersection() {{\n  {}\n}}",
                v.iter().map(|x| indent_d3(x).to_string()).collect::<Vec<_>>().join("\n  ")),
            D3::Minkowski(v) => format!("minkowski() {{\n  {}\n}}",
                v.iter().map(|x| indent_d3(x).to_string()).collect::<Vec<_>>().join("\n  ")),
            D3::Translate(xyz, shape) => format!("translate(v = [{}, {}, {}]) {{\n  {}\n}}", xyz.0, xyz.1, xyz.2, shape.indent()),
            D3::Rotate(xyz, shape) => format!("rotate([{}, {}, {}]) {{\n  {}\n}}", xyz.0, xyz.1, xyz.2, shape.indent()),
            D3::Difference(shape1, shape2) => format!("difference() {{\n  {}\n  {}\n}}", indent_d3(shape1), indent_d3(shape2)),
            D3::Join(name, v) => format!("{}() {{\n  {}\n}}", &name,
                v.iter().map(|x| x.indent().to_string()).collect::<Vec<_>>().join("\n  ")),
            D3::Render(shape) => format!("render() {{\n  {}\n}}", indent_d3(shape)),
        }
    }
    fn indent(&self) -> String {
        self.scad().replace('\n', "\n  ")
    }

}

impl D3 {
    /// Create a cube with side length `s` with lower left corner at the origin.
    pub fn cube<T: Into<X>>(side: T) -> D3 {
        D3::Cube(side.into())
    }

    /// Create a rectangular cuboid with side lengths `x,y,z` with lower left corner at the origin.
    pub fn cuboid<IXYZ: Into<XYZ>>(xyz: IXYZ) -> D3 {
        D3::Cuboid(xyz.into())
    }

    /// Create a sphere with `radius` centered at the origin.
    pub fn sphere_r<T: Into<X>>(radius: T) -> D3 {
        D3::Sphere{radius: radius.into()}
    }

    /// Create a sphere with `diameter` centered at the origin.
    pub fn sphere_d<T: Into<X>>(diameter: T) -> D3 {
        D3::Sphere{radius: diameter.into()/2}
    }

    /// Add (union) two objects
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: D3) -> D3 {
        self + other
    }

    pub fn and(self, other: D3) -> D3 {
        self & other
    }


    /// Render an object to reduce memory usage
    pub fn render(self) -> D3 {
        D3::Render(Box::new(self))
    }

    /// Center an object, if we know how
    pub fn center(self) -> D3 {
        match self {
            D3::Cylinder(h, _d) => self.translate(v3(0,0,-h/2)),
            D3::Cube(x) => self.translate(-v3(x,x,x)/2),
            D3::Cuboid(xyz) => self.translate(-xyz/2),
            D3::LinearExtrude{height: h, twist: t, slices, center: _, shape} 
            => D3::LinearExtrude{height: h, twist: t, slices, center: true, shape},
            _ => self,
        }
    }

    /// Offset shape by circle of radius `r`.
    /// If `r` is positive, equivalent to Minkowski sum with circle of radius `r`.
    pub fn fillet_radius<R: Into<X>>(self, ir: R) -> D3 {
        let r = ir.into();
        self.minkowski(D3::sphere_r(r))
            .invert(1000.)
            .minkowski(D3::sphere_r(r))
            .invert(1000.)
    }

    /// Create a polyhedron from convex hull of vertices.
    pub fn convex_hull<T: Into<XYZ>, I: IntoIterator<Item=T>>(points: I) -> D3 {
        let vertices = points.into_iter().map(|w| {
            let v = w.into(); [v.0 as f64, v.1 as f64, v.2 as f64]
        }).collect::<Vec<[f64; 3]>>();
        let (vert, face) = convex_hull_3d(vertices);
        D3::Polyhedron(
            Box::new(
                vert.into_iter()
                .map(Into::<XYZ>::into)
                .collect::<Vec<XYZ>>()
                ),
            Box::new(face)
            )
    }

    pub fn half_space(aim: Aim) -> D3 {
        match aim {
            Aim::N => D3::cube(MAX).translate(v3(-MAX/2., 0., -MAX/2.)),
            Aim::S => D3::cube(MAX).translate(v3(-MAX/2., -MAX, -MAX/2.)),
            Aim::E => D3::cube(MAX).translate(v3(0., -MAX/2., -MAX/2.)),
            Aim::W => D3::cube(MAX).translate(v3(-MAX, -MAX/2., -MAX/2.)),
            Aim::U => D3::cube(MAX).translate(v3(-MAX/2., -MAX/2., 0.)),
            Aim::D => D3::cube(MAX).translate(v3(-MAX/2., -MAX/2., -MAX)),
            }
    }

    /// Subtract `self` from a cube centered at the origin with edge length `l_edge`.
    // pub fn invert<T: Into<X>>(self, l_edge: T) -> D3 { TODO
    pub fn invert(self, l_edge: f64) -> D3 {
        let shift = -l_edge/2.0;
        D3::cube(l_edge)
            .translate(v3(shift,shift,shift)) 
            // .translate(-0.5*v3(l_edge,l_edge,l_edge)) 
            - self
    }

    /// Create a spheroid with radii, `r1, r2, r3` centered at the origin.
    pub fn spheroid<IXYZ: Into<XYZ>>(radii: IXYZ) -> D3 {
        D3::sphere_r(1).scale3(radii.into())
    }

    pub fn color(self, color_name: ColorEnum) -> D3 {
        D3::Color(color_name, Box::new(self))
    }

    /// Scale size by the factor `s`.
    pub fn scale<T: Into<X>>(self, scale_factor: T) -> D3 {
        D3::Scale(scale_factor.into(), Box::new(self.clone()))
    }

    pub fn scale_x<IX: Into<X>>(&self, scale_factor: IX) -> D3 {
        self.clone().scale3( (scale_factor,1,1) )
    }

    pub fn scale_y<IX: Into<X>>(&self, scale_factor: IX) -> D3 {
        self.clone().scale3( (1,scale_factor,1) )
    }

    pub fn scale_z<IX: Into<X>>(&self, scale_factor: IX) -> D3 {
        self.clone().scale3( (1,1,scale_factor) )
    }

    /// Scale in `x`, `y` and `z` directions.
    pub fn scale3<IXYZ: Into<XYZ>>(self, xyz: IXYZ) -> D3 {
        D3::Scale3(xyz.into(), Box::new(self))
    }

    pub fn mirror<IXYZ: Into<XYZ>>(&self, ixyz: IXYZ) -> D3 {
        D3::Mirror(ixyz.into(), Box::new(self.clone()))
    }

    pub fn translate<IXYZ: Into<XYZ>>(&self, xyz: IXYZ) -> D3 {
        // TODO: Is clone needed here?
        match self {
            D3::Translate(v, d3) => D3::Translate(*v+xyz.into(), d3.clone()),
            _ => D3::Translate(xyz.into(), Box::new(self.clone())),
        }
    }

    pub fn translate_x<T: Into<X>>(self, x: T) -> D3 {
        D3::Translate(v3(x,0,0), Box::new(self))
    }

    pub fn translate_y<T: Into<X>>(self, y: T) -> D3 {
        D3::Translate(v3(0,y,0), Box::new(self))
    }

    pub fn translate_z<T: Into<X>>(self, z: T) -> D3 {
        D3::Translate(v3(0,0,z), Box::new(self))
    }

    pub fn iter_translate<IXYZ: Into<XYZ>>(&self, ixyz: IXYZ, n: u32) -> impl Iterator<Item = D3> + '_ {
        let xyz = ixyz.into();
        (0..n).map(move |ii| self.clone().translate(xyz * ii))
    }


    pub fn rotate<IXYZ: Into<XYZ>>(&self, ixyz: IXYZ) -> D3 {
        D3::Rotate(ixyz.into(), Box::new(self.clone()))
        // match self {
            // D3::Rotate(xyz, d3) => D3::Rotate(*xyz + ixyz.into(), d3.clone()),
            // TODO: These don't commute
            // _ => D3::Rotate(ixyz.into(), Box::new(self.clone())),
        // }
    }

    pub fn rotate_x<IX: Into<X>>(&self, theta: IX) -> D3 {
        self.rotate( (theta,0,0) )
    }

    pub fn rotate_y<IX: Into<X>>(&self, theta: IX) -> D3 {
        self.rotate( (0,theta,0) )
    }

    pub fn rotate_z<IX: Into<X>>(&self, theta: IX) -> D3 {
        self.rotate( (0,0,theta) )
    }

    /// Create a cylinder of height `h` and diameter `d` centered above the XY plane.
    pub fn cylinder_d<H: Into<X>, D: Into<X>>(h: H, d:D) -> D3 {
        D3::Cylinder(h.into(), d.into())
    }

    /// Create a cylinder of height `h` and radius `r` centered above the XY plane.
    pub fn cylinder_r<H: Into<X>, R: Into<X>>(h: H, r:R) -> D3 {
        D3::Cylinder(h.into(), 2*r.into())
    }

    /// Create a frustum of height `h` with starting radius `r0` and ending radius `r1` centered above the XY plane.
    pub fn frustum_r<H: Into<X>, R0: Into<X>, R1: Into<X>>(h: H, r0: R0, r1: R1) -> D3 {
        D3::Frustum(h.into(), 2*r0.into(), 2*r1.into())
    }

    /// Chamfered cylinder
    pub fn chamfer_cylinder_d<H: Into<X>, D: Into<X>, C: Into<X>>(ih: H, id:D, ic:C) -> D3 {
        let h = ih.into();
        let d = id.into();
        let c = ic.into();
        D3::cylinder_d(h-2*c, d)
            .translate_z(c)
            .add(D3::cylinder_d(h, d-2*c))
            .hull()
    }

    /// Chamfered regular polygon prism
    pub fn chamfer_regular_polygon_prism<H: Into<X>, D: Into<X>, C: Into<X>>(n: u32, height: H, diameter:D, chamfer:C) -> D3 {
        let d = diameter.into();
        let c = chamfer.into();
        let h = height.into();

        let outer = D2::regular_polygon(n, d)
            .linear_extrude(h- 2*c)
            .translate_z(c)
            ;
        let inner = D2::regular_polygon(n, d - 2*c/3.0_f64.sqrt())
            .linear_extrude(h)
            ;
        (outer + inner).hull()
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

    pub fn map<F>(self, f: F) -> D3 where F: Fn(D3) -> D3 {
        f(self)
    }
/*
    pub fn iter_map<'a, F>(&'a self, f: F, n: u32) -> impl Iterator<Item = D3> + '_ where F: Fn(D3, u32) + 'a -> D3 {
        (0..n).map(move |ii| f(self.clone(), ii))
    }
*/

    pub fn iter_rotate<IXYZ: Into<XYZ>>(&self, itheta: IXYZ, n: u32) -> impl Iterator<Item = D3> + '_ {
        let theta = itheta.into();
        (0..n).map(move |ii| self.clone().rotate(v3(theta.0 * ii as f32, theta.1 * ii as f32, theta.2 * ii as f32)))
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

    pub fn beveled_box<T: Into<X>>(xyz: XYZ, bevel_in: T) -> D3 {
        let x = xyz.0; 
        let y = xyz.1;
        let z = xyz.2;
        let bevel = bevel_in.into();
        D3::Hull(Box::new(vec![
            D3::cuboid(v3(x,y-bevel*2.,z-bevel*2.)).translate(v3(0.,bevel,bevel)),
            D3::cuboid(v3(x-bevel*2.,y-bevel*2.,z)).translate(v3(bevel,bevel,0.)),
            D3::cuboid(v3(x-bevel*2.,y,z-bevel*2.)).translate(v3(bevel,0.,bevel)),
            ]))
    }

    pub fn beveled_cube_block<T0: Into<X>, T1: Into<X>, T2: Into<X>>(xyz_dim: (u32, u32, u32), i_cube_side: T0, i_bevel: T1, i_gap: T2) -> D3 {
        let cube_side: X = i_cube_side.into(); 
        let bevel: X = i_bevel.into(); 
        let gap: X = i_gap.into(); 
        D3::beveled_box(v3(cube_side-2*gap, cube_side-2*gap, cube_side-2*gap), bevel)
            .translate(v3(gap, gap, gap))
            .iter_translate(v3(cube_side.0, 0., 0.), xyz_dim.0).union()
            .iter_translate(v3(0, cube_side.0, 0.), xyz_dim.1).union()
            .iter_translate(v3(0, 0, cube_side.0), xyz_dim.2).union()
            .add(D3::cuboid(v3(
                        cube_side*xyz_dim.0 - 2*(gap + bevel),
                        cube_side*xyz_dim.1 - 2*(gap + bevel),
                        cube_side*xyz_dim.2 - 2*(gap + bevel)
                        )).translate(v3(gap+bevel,gap+bevel,gap+bevel))
                    )
    }
    
    /// Creates a rounded cube
    ///    1) Centered at the origin
    ///    2) Angle of attack is 30 degrees for the transition from cube face to sphere.
    pub fn rounded_cube<T: Into<X>>(i_side: T) -> D3 {
        let side: X = i_side.into();
        D3::cube(side)
            .translate(v3(-side*0.5,-side*0.5,-side*0.5))
            .intersection(D3::sphere_r(side * (1.0/3.0_f32.sqrt())))
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

    pub fn octahedron(r: f32) -> D3 {
        D3::convex_hull([
            [1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]
        ]).scale(r)
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere() {
        assert_eq!(D3::sphere_r(5).scad(), "sphere(r = 5);");
        assert_eq!(D3::sphere_d(5).scad(), "sphere(r = 2.5);");
    }

    #[test]
    fn test_cylinder() {
        assert_eq!(D3::cylinder_d(10.0, 5).scad(), "cylinder(h = 10, d = 5);");
        assert_eq!(D3::cylinder_r(10.0, 5).scad(), "cylinder(h = 10, d = 10);");
    }

    #[test]
    fn test_cylinder_center() {
        assert_eq!(D3::cylinder_d(8.0, 5).center().scad(), 
                   "translate(v = [0, 0, -4]) {\n  cylinder(h = 8, d = 5);\n}");
    }

    #[test]
    fn test_cube() {
        assert_eq!(D3::cube(9).scad(), "cube(size = 9);");
    }

    #[test]
    fn test_cube_center() {
        assert_eq!(D3::cube(9).center().scad(), "translate(v = [-4.5, -4.5, -4.5]) {\n  cube(size = 9);\n}");

    }

    #[test]
    fn test_cuboid() {
        assert_eq!(D3::cuboid(v3(1,2,3)).scad(),
                   "cube(size = [1, 2, 3]);");
    }

    #[test]
    fn test_cuboid_center() {
        assert_eq!(D3::cuboid(v3(1,2,3)).center().scad(), 
                   "translate(v = [-0.5, -1, -1.5]) {\n  cube(size = [1, 2, 3]);\n}");
    }

    #[test]
    fn test_add() {
        assert_eq!(D3::sphere_r(5).add(D3::cube(9)).scad(),
        "union() {\n  sphere(r = 5);\n  cube(size = 9);\n}");
    }

    #[test]
    fn test_color() {
        assert_eq!(D3::sphere_r(7_i32).add(D3::cube(9)).color(ColorEnum::Red).scad(),
        "color(\"red\") {\n  union() {\n    sphere(r = 7);\n    cube(size = 9);\n  }\n}"
        );
    }
    #[test]
    fn test_iter_translate() {
        assert_eq!(D3::cube(3).iter_translate(v3(1.,2.,3.),4).union().scad(),
            "union() {\n  translate(v = [0, 0, 0]) {\n    cube(size = 3);\n  }\n  translate(v = [1, 2, 3]) {\n    cube(size = 3);\n  }\n  translate(v = [2, 4, 6]) {\n    cube(size = 3);\n  }\n  translate(v = [3, 6, 9]) {\n    cube(size = 3);\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_tuple() {
        assert_eq!(D3::cube(3).rotate((10,20.,30.0)).scad(),
            "rotate([10, 20, 30]) {\n  cube(size = 3);\n}"
        );
    }

    #[test]
    fn test_rotate_array_int() {
        assert_eq!(D3::cube(3).rotate([10,20,30]).scad(),
            "rotate([10, 20, 30]) {\n  cube(size = 3);\n}"
        );
    }

    #[test]
    fn test_rotate_array_float() {
        assert_eq!(D3::cube(3).rotate([10.,20.,30.]).scad(),
            "rotate([10, 20, 30]) {\n  cube(size = 3);\n}"
        );
    }

    #[test]
    fn test_rotate_x() {
        assert_eq!(D3::cube(3).rotate_x(10).scad(),
            "rotate([10, 0, 0]) {\n  cube(size = 3);\n}"
        );
    }

    #[test]
    fn test_rotate_y() {
        assert_eq!(D3::cube(3).rotate_y(20.).scad(),
            "rotate([0, 20, 0]) {\n  cube(size = 3);\n}"
        );
    }

    #[test]
    fn test_rotate_z() {
        assert_eq!(D3::cube(3).rotate_z(30.0).scad(),
            "rotate([0, 0, 30]) {\n  cube(size = 3);\n}"
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

    #[test]
    fn test_convex_hull() {
        assert_eq!(D3::convex_hull([
            v3(  0,  0,  0 ),  //0
            v3( 10,  0,  0 ),  //1
            v3( 10,  7,  0 ),  //2
            v3(  0,  7,  0 ),  //3
            v3(  0,  0,  5 ),  //4
            v3( 10,  0,  5 ),  //5
            v3( 10,  7,  5 ),  //6
            v3(  0,  7,  5 )]  //7
                ).scad(),
            "polyhedron(points = [[0, 0, 0], [10, 0, 0], [10, 7, 0], [0, 7, 0], [0, 0, 5], [10, 0, 5], [10, 7, 5], [0, 7, 5]], faces = [[3, 0, 1, 2], [4, 5, 1, 0], [7, 4, 0, 3], [5, 6, 2, 1], [6, 7, 3, 2], [7, 6, 5, 4]]);"
        );
    }

    #[test]
    fn test_convex_hull_octahedron() {
        assert_eq!(D3::convex_hull([
        [1,1,0], [-1,1,0],[-1,-1,0],[1,-1,0],  // point in xy-plane
        [1,0,1], [-1,0,1],[-1,0,-1],[1,0,-1],  // point in xz-plane
        [0,1,1], [0,-1,1],[0,-1,-1],[0,1,-1],  // point in yz-plane
        ]).scad(),
        "polyhedron(points = [[1, 1, 0], [-1, 1, 0], [-1, -1, 0], [1, -1, 0], [1, 0, 1], [-1, 0, 1], [-1, 0, -1], [1, 0, -1], [0, 1, 1], [0, -1, 1], [0, -1, -1], [0, 1, -1]], faces = [[6, 11, 1], [10, 6, 2], [11, 7, 0], [7, 10, 3], [0, 7, 3, 4], [7, 11, 6, 10], [1, 5, 2, 6], [4, 8, 0], [8, 5, 1], [8, 1, 11, 0], [9, 4, 3], [5, 9, 2], [4, 9, 5, 8], [2, 9, 3, 10]]);"
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(D3::cube(9).scale_x(5).scad(), 
                   "scale(v = [5, 1, 1]) {\n  cube(size = 9);\n}"
        );
        assert_eq!(D3::cube(9).scale_y(5).scad(), 
                   "scale(v = [1, 5, 1]) {\n  cube(size = 9);\n}"
        );
        assert_eq!(D3::cube(9).scale_z(5).scad(), 
                   "scale(v = [1, 1, 5]) {\n  cube(size = 9);\n}"
        );
    }

    #[test]
    fn test_add_map() {
        assert_eq!(D3::cube(9).iter_rotate((10,20,30), 4).union().add_map(|x| x.mirror((1., 0., 0))).scad(),
             "union() {\n  rotate([0, 0, 0]) {\n    cube(size = 9);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 9);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 9);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 9);\n  }\n  mirror(v = [1, 0, 0]) {\n    union() {\n      rotate([0, 0, 0]) {\n        cube(size = 9);\n      }\n      rotate([10, 20, 30]) {\n        cube(size = 9);\n      }\n      rotate([20, 40, 60]) {\n        cube(size = 9);\n      }\n      rotate([30, 60, 90]) {\n        cube(size = 9);\n      }\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_union_union() {
        assert_eq!(D3::cube(9).iter_rotate((10,20,30), 4).union().add(D3::sphere_d(5)).scad(),
        "union() {\n  rotate([0, 0, 0]) {\n    cube(size = 9);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 9);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 9);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 9);\n  }\n  sphere(r = 2.5);\n}"
        );
    }

    #[test]
    fn test_d3_add_op() {
        assert_eq!((D3::cube(9) + D3::sphere_r(5)).scad(),
            "union() {\n  cube(size = 9);\n  sphere(r = 5);\n}"
        );
    }

    #[test]
    fn test_d3_sub_op() {
        assert_eq!((D3::cube(9) - D3::sphere_r(5)).scad(),
            "difference() {\n  cube(size = 9);\n  sphere(r = 5);\n}"
        );
        assert_eq!((D3::cube(9) - D3::spheroid(v3(5,4,3))).scad(),
            "difference() {\n  cube(size = 9);\n  scale(v = [5, 4, 3]) {\n    sphere(r = 1);\n  }\n}"
        );
    }


    #[test]
    fn test_iter_hull() {
        assert_eq!(D3::cube(5).iter_rotate((10,20,30.), 4).hull().scad(),
            "hull() {\n  rotate([0, 0, 0]) {\n    cube(size = 5);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 5);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 5);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_minkowski() {
        assert_eq!(D3::cube(4).iter_rotate((10,20,30), 4).minkowski().scad(),
            "minkowski() {\n  rotate([0, 0, 0]) {\n    cube(size = 4);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 4);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 4);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 4);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_union() {
        assert_eq!(D3::cube(4).iter_rotate((10,20,30), 4).union().scad(),
            "union() {\n  rotate([0, 0, 0]) {\n    cube(size = 4);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 4);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 4);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 4);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_intersection() {
        assert_eq!(D3::cube(4).iter_rotate((10,20,30), 4).intersection().scad(),
            "intersection() {\n  rotate([0, 0, 0]) {\n    cube(size = 4);\n  }\n  rotate([10, 20, 30]) {\n    cube(size = 4);\n  }\n  rotate([20, 40, 60]) {\n    cube(size = 4);\n  }\n  rotate([30, 60, 90]) {\n    cube(size = 4);\n  }\n}"
        );
    }



    #[test]
    fn test_iter_rotate_rotate() {
        assert_eq!(D3::cube(4).iter_rotate((1,2,3), 4).map(move |x| x.rotate((10,20,30))).hull().scad(),
            "hull() {\n  rotate([10, 20, 30]) {\n    rotate([0, 0, 0]) {\n      cube(size = 4);\n    }\n  }\n  rotate([10, 20, 30]) {\n    rotate([1, 2, 3]) {\n      cube(size = 4);\n    }\n  }\n  rotate([10, 20, 30]) {\n    rotate([2, 4, 6]) {\n      cube(size = 4);\n    }\n  }\n  rotate([10, 20, 30]) {\n    rotate([3, 6, 9]) {\n      cube(size = 4);\n    }\n  }\n}"
        );
        assert_eq!(D3::cube(4).iter_rotate((1,2,3), 4).map(move |x| x.rotate_x(90)).hull().scad(),
          "hull() {\n  rotate([90, 0, 0]) {\n    rotate([0, 0, 0]) {\n      cube(size = 4);\n    }\n  }\n  rotate([90, 0, 0]) {\n    rotate([1, 2, 3]) {\n      cube(size = 4);\n    }\n  }\n  rotate([90, 0, 0]) {\n    rotate([2, 4, 6]) {\n      cube(size = 4);\n    }\n  }\n  rotate([90, 0, 0]) {\n    rotate([3, 6, 9]) {\n      cube(size = 4);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_iter_translate_translate() {
        assert_eq!(D3::cube(4).iter_translate((1,2,3), 4).map(move |x| x.translate((10,20,30))).hull().scad(),
            "hull() {\n  translate(v = [10, 20, 30]) {\n    cube(size = 4);\n  }\n  translate(v = [11, 22, 33]) {\n    cube(size = 4);\n  }\n  translate(v = [12, 24, 36]) {\n    cube(size = 4);\n  }\n  translate(v = [13, 26, 39]) {\n    cube(size = 4);\n  }\n}"
        );
    }

}
