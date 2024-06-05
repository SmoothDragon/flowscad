
use tgdscad::*;

fn main() {
    let e = 10.;
    let s = 50.;
    let b = D3::BeveledBox(XYZ(e,e,s), X(1.))
        .translate(XYZ(s/2.-e, s/2.-e, -s/2.))
        .iter_rotate(XYZ(0.,0.,90.), 4)
        .union()
        .iter_rotate(XYZ(0.,90.,0.), 2)
        .union()
        .iter_rotate(XYZ(90.,0.,0.), 2)
        .union()
        ;
    println!("{}", &b);
}
