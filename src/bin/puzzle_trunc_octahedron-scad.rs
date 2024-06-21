
use tgdscad::*;

fn main() {
    let l_edge = 5.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let r_hexagon = 0.75 * 3.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between hexagonal faces
    let t = D3::truncated_octahedron(l_edge);
    let piece = t.clone()
        .add_map(move |x| x.translate(XYZ(2.*r_square, 0., 0.)))
        .add_map(move |x| x.translate(XYZ(0., 2.*r_square, 0.)))
        .add(t.translate(XYZ(r_square, r_square, r_square)))
        ;
    let column = t.clone()
        .iter_translate(XYZ(r_square, r_square, r_square), 5)
        .union()
        .rotate(XYZ(0., 0., 45.))
        .rotate(XYZ((-1.0_f64/3.).acos()*180.0/PI/2., 0., 0.))
        .translate(XYZ(-3.*r_square,0.,0.))
        ;

    println!("{}", &piece.add(column));
}

