use std::fmt;
// use std::ops::Add;


#[derive(Clone, Debug)]
pub struct Circle {
    r: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            r: 10.,
        }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circle(r = {});", self.r)
    }
}


impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "circle(r = {});", r.r),
            Shape::Square(s) => write!(f, "square(size = [{}, {}], center={});", s.x, s.y, s.center),
        }
    }
}
// impl fmt::Debug for Day {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match *self {
            // Day::Monday => write!(f, "Monday"),
            // Day::Tuesday => write!(f, "Tuesday"),
            // Day::Wednesday => write!(f, "Wednesday"),
            // Day::Thursday => write!(f, "Thursday"),
            // Day::Friday => write!(f, "Friday"),
        // }
    // }
// }

#[derive(Clone, Debug)]
pub struct Square {
    x: f32,
    y: f32,
    center: bool,
}

impl Default for Square {
    fn default() -> Self {
        Self {
            x: 10.,
            y: 10.,
            center: false,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "square(size = [{}, {}], center={});", self.x, self.y, self.center)
    }
}

#[derive(Clone, Debug)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[derive(Clone, Debug)]
pub enum D2scad {
    Add { 
        a: Box<D2scad>,
        b: Box<D2scad>,
    },
    Circle {c: Box<Circle>},
    Square {s: Box<Square>},
}


impl D2scad {
    pub fn scad(&self) -> String {
        match &self {
            // D2scad::Shape(Circle(c)) => format!("{}", c),
            // D2scad::Shape::Square(s) => format!("{}", s),
            D2scad::Square{s} => format!("{}", s),
            D2scad::Circle{c} => format!("{}", c),
            D2scad::Add { a, b } => format!("union() {{ {} {} }}", a.scad(), b.scad()),
        }
    }
}

fn main() {
    let c = Circle{r: 1.};
    println!("{}", c);
    println!("{:?}", c);
    let s = Square{ x:2., y:3., ..Default::default()};
    println!("{}", s);
    println!("{:?}", s);
    let c = D2scad::Circle{c: Box::new(Circle{r: 1.})};
    let s = D2scad::Square{s: Box::new(Square{ x: 2., y:3., ..Default::default()})};
    let p = D2scad::Add{a:Box::new(c), b:Box::new(s)};
    // println!("{}", &c.scad());
    println!("{}", &p.scad());
    // println!("{}", s.scad());
    // println!("{:?}", p);
}

