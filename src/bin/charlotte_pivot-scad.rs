use flowscad::*;

fn main() {
    let pivot = D3::cylinder(1.25, 14.25)
        .add(D3::cylinder(9.4, 7.9))
        .add(D3::cylinder(5.15, 10.6))
        .add(D3::cylinder(5.15, 10.6))
        .add(D3::cylinder(1.6, 10.6)
             .translate_z(7.8))
        ;

    println!("$fn=256;\n{}", pivot);

}


