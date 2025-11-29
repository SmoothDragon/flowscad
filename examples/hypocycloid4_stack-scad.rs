use flowscad::*;

fn main() {
    let hypo4: Face = hypocycloid(4, 256)
        .scaled(5.)
        ;

    let n = 100;
    let layers: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo4.clone();
            layer.rotate(Deg(ii as f32));
            layer
        })
        .collect();


    let poly = polygon_stack(layers, 0.2);


    println!("$fn=128;\n{}", poly.scad());
}
