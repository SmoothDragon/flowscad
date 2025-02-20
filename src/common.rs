pub use itertools::Itertools;
pub use core::ops::*;

pub const MAX: f32 = f32::MAX / 10000.;

pub enum ScadParameter {
    R, Radius, 
    IR, InnerRadius,
    OR, OuterRadius,
    D, Diameter, 
    ID, InnerDiameter, 
    OD, OuterDiameter, 
    S, Side,
}

pub trait Radius {
    fn r(&self) -> Self;
}

// pub trait Diameter {
    // fn d(&self, diameter: f32) -> Self where Self: Sized {
        // Self {
            // diameter,
            // ..self
        // }
    // }
// }

// impl Diameter for Circle2 {
    // fn d(self, diameter: f32) -> Self {
        // Self {
            // radius: diameter/2.,
            // ..self
        // }
    // }
// }


pub trait SCAD {
    fn scad(&self) -> String;
    fn indent(&self) -> String;
}

/// Methods for creating an SCAD object from an iterator of SCAD objects.
pub trait DIterator<T> : Iterator<Item=T> {
    fn hull(self) -> T where Self: Iterator<Item = T>;
    fn union(self) -> T where Self: Iterator<Item = T>;
    fn intersection(self) -> T where Self: Iterator<Item = T>;
    fn minkowski(self) -> T where Self: Iterator<Item = T>;
}

#[derive(Clone, Debug)]
pub enum ColorEnum {
    Blue,
    Green,
    Red,
}


pub trait PairedIterator<T>: IntoIterator<Item = T> {
    fn pairs(self) -> impl Iterator<Item = (T, T)> where Self: IntoIterator<Item = T>;
}

impl<T: Clone, I: IntoIterator<Item=T>> PairedIterator<T> for I {  
    fn pairs(self) -> impl Iterator<Item = (I::Item, I::Item)> 
    where 
        Self: IntoIterator<Item = T>
    {
        let mut pk = self.into_iter().peekable();
        let first = pk.peek().unwrap().clone();
        pk.chain(std::iter::once(first)).tuple_windows()
    }
}

pub fn arange(a: f64, b: f64, n: usize) -> impl Iterator<Item = f64> {
    let step = (b-a) / (n as f64);
    (0..n).map(move |x| a + (x as f64)*step)
}

pub fn linspace_f32(a: f32, b: f32, n: usize) -> impl Iterator<Item = f32> {
    let step = (b-a) / (n as f32);
    (0..=n).map(move |x| a + (x as f32)*step)
}

pub fn linstep_f32(a: f32, b: f32, n: usize) -> impl Iterator<Item = f32> {
    let step = (b-a) / (n as f32);
    (0..=n).map(move |x| a + (x as f32)*step)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pairs() {
        assert_eq!([1,5,3,2].pairs().collect::<Vec<_>>(), [(1,5), (5,3), (3,2), (2,1)]);
        assert_eq!([(0,0), (0,1), (1,1), (1,0)].pairs().collect::<Vec<_>>(),
            [((0, 0), (0, 1)), ((0, 1), (1, 1)), ((1, 1), (1, 0)), ((1, 0), (0, 0))]);
    }
}
