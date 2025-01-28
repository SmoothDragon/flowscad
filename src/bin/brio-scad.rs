use flowscad::*;

fn main() {
    let d = 6.;
    let l = 15;

    let result = D3::cylinder_d(l, d)
        .translate_y(d/2.)
        .add(D3::cuboid( (d/2., d, l) ))
        .rotate_y(90)
        ;

    println!("$fn=256;\n{}", result);
}

