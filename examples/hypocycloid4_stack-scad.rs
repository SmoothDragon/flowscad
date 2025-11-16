use flowscad::*;

fn main() {
    let hypo4: Face = hypocycloid(4, 8);

    let mut layers: Vec<Face> = Vec::new();
    layers.push(hypo4.clone());
    layers.push(hypo4.rotated(Deg(-45.)));

    let poly = polygon_stack(layers, 1.);


    println!("$fn=128;\n{}", poly.scad());
}
