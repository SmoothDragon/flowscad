// This replaces a plastic insert for the shower head.
// Above the XY plane will be something equivalent to the old piece.
// Below the XY plane will be what is added on.

use flowscad::*;

fn main() {
    let extend = 10.;
    let h_center = 54.+extend;
    let d_center = 25.2;
    let h_outer = 30.+extend;
    let d_outer = 31.6;
    let h_rim = 3.28+extend;
    let d_rim = 33.5;
    let h_notch = 4.; // 3.5 away from perimter
    let w_notch = 7.;
     
    let center_column = D3::cylinder_r(h_center, d_center/2.)
        .add(D3::cylinder_r(h_outer, d_outer/2.))
        .add(D3::cylinder_r(h_rim, d_rim/2.))
        .add(D3::cuboid(v3(h_notch, w_notch, h_outer)).translate(v3(d_outer/2.-0.5, -w_notch/2., 0.)))
        .difference(D3::cylinder_r(h_center+2., d_center/2.-2.).translate(v3(0.,0.,-1.)))
        .difference(D3::cuboid(v3(2.*d_center, 14., 3.7*2.)).translate(v3(-d_center, -7., h_center-3.7)))
        ;

    println!("{}", &center_column);
}
