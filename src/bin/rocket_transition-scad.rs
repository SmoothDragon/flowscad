
use tgdscad::*;

fn main() {
    let upper_b = 4.34;
    let upper_h = 44.45;
    let lower_b = 9.65;
    let lower_h = 57.15;
    let inner_d = 103.1/2.;
    let upper = D2::triangle(v2(0.,0.), v2(upper_b, 0.), v2(0., upper_h))
        .translate(v2(inner_d, 0.))
        .rotate_extrude(360)
        ;

    let lower = D2::polygon(vec![v2(0.,0.), v2(lower_b, 0.), v2(upper_b, lower_h), v2(0., lower_h)])
        .translate(v2(inner_d, 0.))
        .rotate_extrude(360)
        ;

    println!("$fn=256;\n{}", &upper.translate(2.*inner_d,2.*inner_d, 0.).add(lower));
}
