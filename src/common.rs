pub trait SCAD {
    fn scad(&self) -> String;
    fn indent(&self) -> String;
}

/// Methods for creating an SCAD object from an iterator of SCAD objects.
pub trait DIterator<T> : Iterator<Item=T> {
    fn hull(self: Self) -> T where Self: Iterator<Item = T>;
    fn union(self: Self) -> T where Self: Iterator<Item = T>;
    fn intersection(self: Self) -> T where Self: Iterator<Item = T>;
    fn minkowski(self: Self) -> T where Self: Iterator<Item = T>;
}

#[derive(Clone, Debug)]
pub enum ColorEnum {
    Blue,
    Green,
    Red,
}

