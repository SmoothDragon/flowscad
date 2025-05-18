//! Create OpenSCAD files using Rust.

use crate::*;
use anyhow::Result;

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

impl Add<D2> for D2 {
    type Output = D2;

    fn add(self, other: D2) -> D2 {
        match self { // Combine Unions if possible
            D2::Join("union", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("union", vec)
                },
            _ => D2::Join("union", Box::new(vec![self, other])),
        }
    }
}

impl Sub<D2> for D2 {
    type Output = D2;

    fn sub(self, other: D2) -> D2 {
        D2::Difference(Box::new(self), Box::new(other))
    }
}

impl BitAnd<D2> for D2 {
    type Output = D2;

    fn bitand(self, other: D2) -> D2 {
        match self { // Combine intersections if possible
            D2::Join("intersection", vec) => {
                let mut vec = vec;
                vec.push(other);
                D2::Join("intersection", vec)
                },
            _ => D2::Join("intersection", Box::new(vec![self, other])),
        }
    }
}

impl AddAssign for D2 {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl SubAssign for D2 {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
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
    // Path(X, Box<Vex<XY>),
    // Cycle(Box<Vex<XY>, X),
    Polygon(Box<Vec<XY>>),
    Text(String),
    Color(ColorEnum, Box<D2>),
    Rotate(X, Box<D2>),
    Scale(X, Box<D2>),
    ScaleXY(XY, Box<D2>),
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

// impl<Borrowed: ?Sized> std::borrow::Borrow<Borrowed> for D2 {  
    // fn borrow(&self) -> &Self {
        // self
    // }
// }

// impl<'a> Borrow<MyTrait + 'a> for MyStruct {
    // fn borrow(&self) -> &(MyTrait + 'a) {
        // self
    // }
// }

pub fn indent(shape: &D2) -> String {
    format!("{}", shape).replace('\n', "\n  ")
}



/** TODO
pub fn sector2(r: f32, theta: f32) -> D2 {
    if r < 180.0 {
        D2::half_plane(Aim::S)
            .rotate(theta)
            .intersection(D2::half_plane(Aim::N))
            .intersection(circle2().r(r))
    } else {
        D2::half_plane(Aim::S)
            .rotate(theta)
            .add(D2::half_plane(Aim::N))
            .intersection(circle2().r(r))
    }
}
*/


impl D2 {
    /// Create a circle using variable inputs centered at the origin.
    // pub fn circle<T: Into<X>>(diameter: T) -> D2 {
        // D2::Circle(diameter.into())
    // }
    
    /// Set outer diameter of shape
    pub fn od<T: Into<X>>(self, diameter: T) -> D2 {
        match self {
            D2::Circle(_) => D2::Circle(diameter.into()),
            D2::Square(_) => D2::Square(diameter.into() * 2.0_f32.powf(-0.5)),
            _ => self,
        }
    }
            // D2::Square(size) => format!("square(size = {});", size),
            // D2::Rectangle(XY(x,y)) => format!("square(size = [{}, {}]);", x, y),
            // D2::Text(letters) => format!("text(\"{}\", font=\"Liberation Sans\");", letters),

    /// Create a circle of `diameter` centered at the origin.
    pub fn circle_d<T: Into<X>>(diameter: T) -> D2 {
        D2::Circle(diameter.into())
    }

    /// Create a circle of `radius` centered at the origin.
    pub fn circle_r<IX: Into<X>>(radius: IX) -> D2 {
        D2::Circle(2*radius.into())
    }

    pub fn chamfered_circle_r<IX: Into<X>>(r: IX) -> D2 {
        let r: X = r.into();
        D2::square(r)
            .rotate(-135)
            .intersection(D2::rectangle( (4.*r, 2.*r) ).center())
            .add(D2::circle_r(r))
    }
    
    pub fn circle_chain(n: u32) -> D2 {
        let chain = D2::circle_d(2.)
            .iter_translate([3.0_f32.sqrt()/2., 0.], n)
            .union()
            .offset_radius(-0.5)
            ;
        let _gap_fill = D2::circle_d(1.5)
            .difference(D2::circle_d(1))
            .intersection(D2::square(2.).rotate(45))
            .translate( (3.0_f32.sqrt()/4., -1.75) )
            .iter_translate([3.0_f32.sqrt()/2., 0.], n-1)
            .union()
            // .mirror( (0,1) )
            ;
        chain
    }

    /// Create a square with side length `side` with lower left corner at the origin.
    pub fn square<T: Into<X>>(side: T) -> D2 {
        D2::Square(side.into())
    }

    // pub fn square(side: f32) -> Option<D2> {
        // D2::

    /// Create a rectangle with lower left corner at the origin.
    pub fn rectangle<IXY: Into<XY>>(xy: IXY) -> D2 {
        D2::Rectangle(xy.into())
    }

    /// Create a rectangle with lower left corner at the origin.
    pub fn dwedge<IR: Into<X>, IX: Into<X>>(ir: IR, ix: IX) -> D2 {
        let r = ir.into();
        let x = ix.into();
        let base = D2::Circle(2*r);
        if x.0 <= 90. {
            base.and(D2::Square(x).and(D2::Square(x).rotate(x-90)))
        } else {
            base.and(D2::Square(x).add(D2::Square(x).rotate(x-90)))
        }
    }

    /// Create an egg shape from one variable.
    ///     r = radius of semicircle on one side
    ///     2r = radius of transition curve
    ///     smaller radius end cap
    pub fn egg<IR: Into<X>>(ir: IR) -> D2 {
        let r: X = ir.into();
        let r2 = (2.0-2.0_f32.sqrt())*r;
        D2::circle_r(r)
            + D2::sector(2*r, 45).translate_x(-r)
            + D2::sector(2*r, 45).translate_x(-r).mirror((1,0))
            + D2::circle_r(r2).translate_y(r)
    }

    pub fn text(letters: String) -> D2 {
        D2::Text(letters)
    }

    /// Create a line segment from (x0,y0) to (x1,y1) of width w
    pub fn line<IXY0: Into<XY>, IXY1: Into<XY>, W: Into<X>>(ixy0: IXY0, ixy1: IXY1, iw: W) -> D2 {
        let XY(x0, y0) = ixy0.into();
        let XY(x1, y1) = ixy1.into();
        let w = iw.into();
        let angle = if x0 == x1 {
            X(90.)
        } else {
            ((y1-y0)/(x1-x0)).atan()*180.0/PI
        };
        let length = ((x1-x0).powf(2.0) + (y1-y0).powf(2.0)).powf(0.5);
        D2::rectangle( (length, w) )
            .translate_y(-w/2)
            .rotate(angle)
            .translate( if x0<x1 { XY(x0, y0) } else { XY(x1, y1) } )
    }

    pub fn cycle<IX: Into<X>>(points: Vec<XY>, width: IX) -> D2 {
        let w = width.into();
        points.into_iter()
            .pairs()
            .map(|(a,b)| D2::line(a, b, w))
            .union()
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

    pub fn difference(self, other: D2) -> D2 {
        self - other
    }

    pub fn half_plane(aim: Aim) -> D2 {
        match aim {
            Aim::N => D2::square(MAX2).translate( (-MAX2/2., 0) ),
            Aim::S => D2::square(MAX2).translate( (-MAX2/2., -MAX2) ),
            Aim::E => D2::square(MAX2).translate( (0, -MAX2/2.) ),
            Aim::W => D2::square(MAX2).translate( (-MAX2, -MAX2/2.) ),
            Aim::U => D2::square(MAX2).translate( (-MAX2/2., 0) ),
            Aim::D => D2::square(MAX2).translate( (-MAX2/2., -MAX2) ),
            // Aim::Angle(theta) => D2::Square(StrictlyPositiveFinite(MAX2)).translate(XY(0., -MAX2/2.)).rotate(*theta),
            }
    }

    pub fn add_map<F>(self, f: F) -> D2 where F: Fn(D2) -> D2 {
        self.clone().add(f(self))
    }

    pub fn repeat_map<F>(self, n:u32, f: F) -> D2 where F: Fn(D2) -> D2 {
        let mut result = self;
        for _ in 0..n {
            result = f(result);
        }
        result
    }

    pub fn map<F>(self, f: F) -> D2 where F: Fn(D2) -> D2 {
        f(self)
    }

    pub fn hull(self) -> D2 {
        match self { // Combine Unions if possible
            D2::Join("union", vec) => D2::Join("hull", vec),
            _ => D2::Join("hull", Box::new(vec![self])),
        }
    }

    pub fn intersection(self, other: D2) -> D2 {
        self & other
    }

    pub fn and(self, other: D2) -> D2 {
        self & other
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

    pub fn triangle<P1: Into<XY>, P2: Into<XY>, P3: Into<XY>>(xy0: P1, xy1: P2, xy2: P3) -> D2 {
        D2::Polygon(Box::new(vec![xy0.into(), xy1.into(), xy2.into()]))
    }

    pub fn polygon(points: Vec<XY>) -> D2 {
        D2::Polygon(Box::new(points))
    }

    /// Create a polygon from convex hull of vertices.
    pub fn convex_hull<T: Into<XY>, I: IntoIterator<Item=T>>(points: I) -> D2 {
        let vertices = points.into_iter().map(|w| {
            let v = w.into(); [v.0 as f32, v.1 as f32]
        }).collect::<Vec<[f32; 2]>>();
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
        let theta: X = 360. / Into::<X>::into(num_sides);
        D2::convex_hull(
            (0..num_sides)
            .map(|ii| Into::<[f32; 2]>::into(r.rotate_deg(ii * theta)))
            .collect::<Vec<[f32; 2]>>()
            )
    }

    pub fn hexagon<IX: Into<X>>(o_radius: IX) -> D2 {
        let r: XY = v2(o_radius.into(), 0);
        let theta = X(60.);
        D2::convex_hull(
            (0..6)
            .map(|ii| Into::<[f32; 2]>::into(r.rotate_deg(ii * theta)))
            .collect::<Vec<[f32; 2]>>()
            )
    }

    pub fn hexagram<IX: Into<X>>(r: IX) -> D2 {
        D2::regular_polygon(3, r)
            .add_map(|x| x.rotate(180))
    }

    pub fn koch_snowflake<IX: Into<X>>(r: IX, iter: u32) -> D2 {
        let r = r.into();
        D2::hexagram(r) 
            .rotate(90)
            .repeat_map(iter, |x| x.add_map(|y|
                y.scale(1./3.)
                .translate_y(2.*r/3.)
                .iter_rotate_equal(6)
                .union()
                )
            )
    }

    pub fn pentagram<IX: Into<X>>(r: IX) -> D2 {
        let r: X = r.into();
        let wedge = D2::square(r);
        wedge.clone().rotate(18)
            .and(wedge.clone().rotate(72))
            .and(wedge.clone().translate_x(-r/2))
            .translate_y(-r)
            .rotate(180)
            .iter_rotate_equal(5)
            .union()
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


    pub fn mirror<IXY: Into<XY>>(&self, xy: IXY) -> D2 {
        D2::Mirror(xy.into(), Box::new(self.clone()))
    }

    // pub fn iter_translate(&self, xy: XY, n: u32) -> impl Iterator<Item = D2> + '_ {
        // (0..n).map(move |ii| self.translate(v2(xy.0 * ii as f32, xy.1 * ii as f32)))
    // }

    pub fn iter_translate<IXY: Into<XY>>(&self, ixy: IXY, n: u32) -> impl Iterator<Item = D2> + '_ {
        let xy = ixy.into();
        (0..n).map(move |ii| self.clone().translate(xy * ii))
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

    pub fn iter_cyclic(&self, n: u32) -> impl Iterator<Item = D2> + '_ {
        (0..n).map(move |ii| self.rotate(360./(n as f64) * ii as f64))
    }

    pub fn iter_dihedral(&self, n: u32) -> impl Iterator<Item = D2> + '_ {
        (0..n).flat_map(move |ii| { 
            let obj = self.rotate(360./(n as f64) * ii as f64);
            [obj.clone(), obj.mirror(XY(0.,1.))]
            })
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
    pub fn scale_xy<IXY: Into<XY>>(self, xy: IXY) -> D2 {
        D2::ScaleXY(xy.into(), Box::new(self))
    }

    /// Scale only in `x` direction.
    pub fn scale_x<IX: Into<X>>(self, x: IX) -> D2 {
        self.scale_xy( (x, 1) )
    }

    /// Scale only in `y` direction.
    pub fn scale_y<IX: Into<X>>(self, y: IX) -> D2 {
        self.scale_xy( (1, y) )
    }

    pub fn linear_extrude<IX: Into<X>>(&self, i_height: IX) -> D3 {
        let height = i_height.into();
        D3::LinearExtrude{height, twist: X(0.), slices: 0, center: false, shape: Box::new(self.clone())}
    }

    pub fn linear_extrude_extra<IX: Into<X>, IT: Into<X>>(&self, i_height: IX, i_twist: IT, slices: u32) -> D3 {
        let height = i_height.into();
        let twist = i_twist.into();
        D3::LinearExtrude{height, twist, slices, center: false, shape: Box::new(self.clone())}
    }

    pub fn rotate_extrude<IX: Into<X>>(&self, x: IX) -> D3 {
        D3::RotateExtrude(x.into(), Box::new(self.clone()))
    }

    /// Intended to be transition from print bed.
    pub fn chamfer45<IX: Into<X>>(&self, h: IX) -> D3 {
        let h = h.into();
        let h_layer = X(0.1);
        let layers = unsafe { (h / h_layer).0.to_int_unchecked::<i32>() };
        // let layers = h * 10;
        if layers >= 0 {
            (0..layers).
                map(|x| self.clone()
                    .offset_chamfer(-h_layer*x)
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
        } else {
            (0..-layers).
                map(|x| self.clone()
                    .offset_chamfer(h_layer*x)
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
        }
    }


    /// Intended to be transition from print bed.
    /// Start at a 45 angle, and then transition to a circular arc
    /// at the tangent point.
    pub fn chamfer45_round<IX: Into<X>>(&self, h: IX) -> D3 {
        let h: X = h.into();
        let h_layer = X(0.1);
        let layers = unsafe { (h / h_layer).0.to_int_unchecked::<i32>() };
        let layers_arc = unsafe { ((h / h_layer)*0.707).0.to_int_unchecked::<i32>() };
        if layers >= 0 {
            (0..layers_arc)
                .map(|x| X(x as f32))
                .map(|x| self.clone()
                    .offset_chamfer(-h*(1-(1-(x/layers)*(x/layers)).sqrt()))
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
            + (layers_arc..layers)
                .map(|x| self.clone()
                    .offset_chamfer(-h*((Into::<X>::into(x))/layers as f32 - 0.414))
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
        } else {
            (0..(-layers+layers_arc))
                .map(|x| self.clone()
                    .offset_chamfer(h*((Into::<X>::into(x))/layers as f32))
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
            + ((-layers+layers_arc)..-layers)
                .map(|x| X(x as f32))
                .map(|x| self.clone()
                    .offset_chamfer(-h*((1-(1+x/layers)*(1+x/layers)).sqrt()-0.414))
                    .linear_extrude(h_layer)
                    .translate_z(h_layer*x)
                    )
                .union()
        }
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
            // D2::Text(letters) => format!("text(\"{}\", font=\"Liberation Sans\");", letters),
            D2::Text(letters) => format!("text(\"{}\", font=\"B612 Mono\", halign=\"center\", valign=\"center\");", letters),
            D2::RoundedRectangle(XY(x,y), r) => format!("{};",
                D2::circle_r(*r)
                    .add_map(move |shape| shape.translate_x(*x - 2 * *r))
                    .add_map(move |shape| shape.translate_y(*y - 2 * *r))
                    .hull()
                    .translate(v2(*r,*r))
                ),
            // D2::Path(width, points) => format!("polygon(points = [ {} ]);",
                // points.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ")),
            D2::Polygon(points) => format!("polygon(points = [ {} ]);",
                points.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ")),
            D2::Color(color, shape) => format!("color({}) {{\n  {}\n}}",
                match color {
                    ColorEnum::Blue => "\"blue\"",
                    ColorEnum::Green => "\"green\"",
                    ColorEnum::Red => "\"red\"",
                    ColorEnum::Black => "\"black\"",
                    ColorEnum::Yellow => "\"yellow\"",
                    ColorEnum::Orange => "\"orange\"",
                    ColorEnum::Lime => "\"lime\"",
                    ColorEnum::Cyan => "\"cyan\"",
                    ColorEnum::Pink => "\"pink\"",
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
            D2::ScaleXY(XY(x,y), shape) => format!("scale(v = [{}, {}]) {{\n  {}\n}}", x, y, indent(shape)),
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

    #[test]
    fn test_circle() {
        assert_eq!(D2::circle_d(5).scad(), "circle(d = 5);");
        assert_eq!(D2::circle_r(5).scad(), "circle(d = 10);");
        assert_eq!(D2::circle_r(0).od(10).scad(), "circle(d = 10);");
    }

    #[test]
    fn test_square() {
        assert_eq!(D2::square(9).scad(), "square(size = 9);");
        assert_eq!(D2::square(0).od(10.0*2.0_f32.sqrt()).scad(), "square(size = 10);");
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
    fn test_line() {
        assert_eq!(D2::line( (0,0), (1,1), 1 ).scad(),
            "translate(v = [0, 0]) {\n  rotate(45) {\n    translate(v = [0, -0.5]) {\n      square(size = [1.4142135, 1]);\n    }\n  }\n}"
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
        assert_eq!(D2::circle_d(5).iter_translate(v2(1.,2.),4).union().scad(),
            "union() {\n  translate(v = [0, 0]) {\n    circle(d = 5);\n  }\n  translate(v = [1, 2]) {\n    circle(d = 5);\n  }\n  translate(v = [2, 4]) {\n    circle(d = 5);\n  }\n  translate(v = [3, 6]) {\n    circle(d = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate() {
        assert_eq!(D2::square(9).iter_rotate(20, 4).sum::<D2>().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(D2::square(9).iter_rotate(20, 4).product::<D2>().scad(),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(D2::square(9).iter_rotate(20, 4).union().scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_add_map() {
        assert_eq!(D2::square(9).iter_rotate(20, 4).union().add_map(|x| x.mirror(v2(1., 0.))).scad(),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n  mirror(v = [1, 0]) {\n    union() {\n      rotate(0) {\n        square(size = 9);\n      }\n      rotate(20) {\n        square(size = 9);\n      }\n      rotate(40) {\n        square(size = 9);\n      }\n      rotate(60) {\n        square(size = 9);\n      }\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_union_union() {
        assert_eq!(D2::square(9).iter_rotate(20, 4).union().add(D2::circle_d(5)).scad(),
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

    // #[test]
    // fn test_d3_add_op() {
        // assert_eq!((D3::cube(9) + D3::sphere_r(5)).scad(),
            // "union() {\n  cube(size = 9);\n  sphere(r = 5);\n}"
        // );
    // }

    // #[test]
    // fn test_d3_sub_op() {
        // assert_eq!((D3::cube(9) - D3::spheroid(v3(5,4,3))).scad(),
            // "difference() {\n  cube(size = 9);\n  scale(v = [5, 4, 3]) {\n    sphere(r = 1);\n  }\n}"
        // );
    // }

    #[test]
    fn test_iter_hull() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).hull()),
            "hull() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_minkowski() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).minkowski()),
            "minkowski() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_union() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).union()),
            "union() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_intersection() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).intersection()),
            "intersection() {\n  rotate(0) {\n    square(size = 9);\n  }\n  rotate(20) {\n    square(size = 9);\n  }\n  rotate(40) {\n    square(size = 9);\n  }\n  rotate(60) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_linear_extrude() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).intersection().linear_extrude(10)),
    "linear_extrude(height = 10, twist = 0, slices = 0, center = false, convexity=50) {\n  intersection() {\n    rotate(0) {\n      square(size = 9);\n    }\n    rotate(20) {\n      square(size = 9);\n    }\n    rotate(40) {\n      square(size = 9);\n    }\n    rotate(60) {\n      square(size = 9);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_rotate_extrude() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).intersection().rotate_extrude(180)),
            "rotate_extrude(angle = 180) {\n  intersection() {\n    rotate(0) {\n      square(size = 9);\n    }\n    rotate(20) {\n      square(size = 9);\n    }\n    rotate(40) {\n      square(size = 9);\n    }\n    rotate(60) {\n      square(size = 9);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_iter_rotate_rotate() {
        assert_eq!(format!("{}", D2::square(9).iter_rotate(20, 4).map(move |x| x.rotate(10)).hull()),
            "hull() {\n  rotate(10) {\n    square(size = 9);\n  }\n  rotate(30) {\n    square(size = 9);\n  }\n  rotate(50) {\n    square(size = 9);\n  }\n  rotate(70) {\n    square(size = 9);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_translate_translate() {
        assert_eq!(D2::circle_d(5).iter_translate(v2(1.,2.),4).map(move |x| x.translate(v2(-1., -1.))).union().scad(),
            "union() {\n  translate(v = [-1, -1]) {\n    circle(d = 5);\n  }\n  translate(v = [0, 1]) {\n    circle(d = 5);\n  }\n  translate(v = [1, 3]) {\n    circle(d = 5);\n  }\n  translate(v = [2, 5]) {\n    circle(d = 5);\n  }\n}"
        );
    }

    #[test]
    fn test_iter_dihedral() {
        assert_eq!(D2::rectangle( (5,1) ).iter_dihedral(4).union().scad(),
          "union() {\n  rotate(0) {\n    square(size = [5, 1]);\n  }\n  mirror(v = [0, 1]) {\n    rotate(0) {\n      square(size = [5, 1]);\n    }\n  }\n  rotate(90) {\n    square(size = [5, 1]);\n  }\n  mirror(v = [0, 1]) {\n    rotate(90) {\n      square(size = [5, 1]);\n    }\n  }\n  rotate(180) {\n    square(size = [5, 1]);\n  }\n  mirror(v = [0, 1]) {\n    rotate(180) {\n      square(size = [5, 1]);\n    }\n  }\n  rotate(270) {\n    square(size = [5, 1]);\n  }\n  mirror(v = [0, 1]) {\n    rotate(270) {\n      square(size = [5, 1]);\n    }\n  }\n}"
        );
    }

    #[test]
    fn test_triangle() {
        assert_eq!(D2::triangle((0.,0.), (1., 0.), (0., 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::triangle((0,0), v2(1., 0.), (0, 1.)).scad(),
            "polygon(points = [ [0, 0], [1, 0], [0, 1] ]);");
        assert_eq!(D2::triangle((0,0), [1., 0.], [0, 1]).scad(),
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
