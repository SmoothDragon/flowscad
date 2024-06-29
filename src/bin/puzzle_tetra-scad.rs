
use tgdscad::*;

fn main() {
    let e = 10.;
    let s = 50.;
    let b = D3::beveled_box(v3(e,e,s), 1.)
        .translate(v3(s/2.-e, s/2.-e, -s/2.))
        .iter_rotate(v3(0.,0.,90.), 4)
        .union()
        .iter_rotate(v3(0.,90.,0.), 2)
        .union()
        .iter_rotate(v3(90.,0.,0.), 2)
        .union()
        ;
    let sqrt2 = f64::powf(2.,0.5);
    let t_edge = (s-2.*e)*sqrt2;
    let t = D3::cuboid(t_edge, 0.01, 0.01)
        .translate(v3(-t_edge/2., 0., t_edge/2./sqrt2))
        .add_map(|x| x.rotate(v3(180., 0., 90.)))
        .hull()
        ;

    println!("{}", &b.add(t.translate(v3(s,0.,0.))));
    // println!("{}", &t);
}
