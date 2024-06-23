// This replaces a plastic insert for the shower head.
// Above the XY plane will be something equivalent to the old piece.
// Below the XY plane will be what is added on.

use tgdscad::*;

fn main() {
    let r = 8;
    let R = 50.;
    let h = 100;
     
    let holder = D3::cylinder(h, r)
        .rotate(XYZ(45., 0., 0.))
        .translate(R, 0., -10.)
        .add(D3::cylinder(h, r))
        .hull()
        .iter_rotate(XYZ(0.,0.,15.), 24)
        .union()
        .intersection(D3::cuboid(200,200,30).translate(-100.,-100.,0.))
        ;

    println!("$fn=128;\n{}", &holder);
}
