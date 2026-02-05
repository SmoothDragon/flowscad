use flowscad::*;

fn main() {
    let square = line_segment(C32::new(10.,0.), C32::new(0.,10.), 0.8);
    println!("{}", square.scad());
}
