pub trait SCAD {
    fn scad(&self) -> String;
    fn indent(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum ColorEnum {
    Blue,
    Green,
    Red,
}

