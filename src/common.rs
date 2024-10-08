use itertools::Itertools;

pub const MAX: f32 = f32::MAX / 10000.;

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
