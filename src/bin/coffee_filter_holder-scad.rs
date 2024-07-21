// This replaces a plastic insert for the shower head.
// Above the XY plane will be something equivalent to the old piece.
// Below the XY plane will be what is added on.

use flowscad::*;

fn main() {
    let r = 8;
    let R = 37.;
    let h = 100;
     
    let holder = D3::cylinder(h, r)
        .rotate(v3(30., 0., 30.))
        .translate(v3(R, 0, -10))
        .add(D3::cylinder(h, r))
        .hull()
        .iter_rotate(v3(0.,0.,15.), 24)
        .union()
        .intersection(D3::cuboid(v3(200,200,40)).translate(v3(-100.,-100.,0.)))
        ;

    println!("$fn=128;\n{}", &holder);
}
