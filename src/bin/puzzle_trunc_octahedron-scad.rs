
use tgdscad::*;


pub fn circle_beveled_box(xyz: XYZ, bevel: f64) -> D3 {
    let x = xyz.0; 
    let y = xyz.1;
    let z = xyz.2;
    let rod = D3::cylinder(x.max(y).max(z), bevel);
    D3::cuboid(x,y,z) - rod
}

pub fn beveled_truncated_octahedron(l_edge: f64) -> D3 {
    //* Create a beveled truncated ocatahedron with edge length `l_edge` centered at the origin
    let bevel = 0.5;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    D3::Hull(Box::new(vec![
        D3::beveled_box(XYZ(l_edge, l_edge, 2.0*r_square), bevel)
            .translate(-l_edge/2.0, -l_edge/2.0, -r_square)
            .rotate(0., 0., 45.),
        D3::beveled_box(XYZ(l_edge, 2.*r_square, l_edge), bevel)
            .translate(-l_edge/2.0, -r_square, -l_edge/2.0)
            .rotate(0., 45., 0.),
        D3::beveled_box(XYZ(2.*r_square, l_edge, l_edge), bevel)
            .translate(-r_square, -l_edge/2.0, -l_edge/2.0)
            .rotate(45., 0., 0.),
        ]))
}

fn main() {
    let l_edge = 5.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let r_hexagon = 0.75 * 3.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between hexagonal faces
    let t = beveled_truncated_octahedron(l_edge);
    let bevel = 1.;
    let piece = t.clone()
        .add_map(move |x| x.translate(2.*r_square, 0., 0.))
        .add_map(move |x| x.translate(0., 2.*r_square, 0.))
        .add(t.clone().translate(r_square, r_square, r_square))
        ;
    let column = t.clone()
        .iter_translate(XYZ(r_square, r_square, r_square), 5)
        .union()
        .rotate(0., 0., 45.)
        .rotate((-1.0_f64/3.).acos()*180.0/PI/2., 0., 0.)
        .translate(-3.*r_square,0.,0.)
        ;

    // println!("{}", &piece.add(column));
    println!("$fn=64;\n{}", circle_beveled_box(XYZ(l_edge, 2.*r_square, l_edge), bevel));
}

