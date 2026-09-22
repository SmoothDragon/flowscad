use flowscad::*;

fn main() {
    let hypo4: Face = hypocycloid(4, 256);

    println!("$fn=128;\n{}", hypo4.scad());
}
