
use flowscad::*;


// pub fn circle_beveled_box(xyz: Real3, bevel: f64) -> D3 {
    // let x = xyz.0; 
    // let y = xyz.1;
    // let z = xyz.2;
    // let rod = D3::cylinder(x.max(y).max(z), bevel);
    // D3::cuboid(x,y,z) - rod
// }

pub fn beveled_truncated_octahedron(l_edge: f32) -> D3 {
    //* Create a beveled truncated ocatahedron with edge length `l_edge` centered at the origin
    let bevel = 0.5;
    let r_square = 2.0_f32.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    D3::Hull(Box::new(vec![
        D3::beveled_box(v3(l_edge, l_edge, 2.0*r_square), bevel)
            .translate(v3(-l_edge/2.0, -l_edge/2.0, -r_square))
            .rotate(v3(0., 0., 45.)),
        D3::beveled_box(v3(l_edge, 2.*r_square, l_edge), bevel)
            .translate(v3(-l_edge/2.0, -r_square, -l_edge/2.0))
            .rotate(v3(0., 45., 0.)),
        D3::beveled_box(v3(2.*r_square, l_edge, l_edge), bevel)
            .translate(v3(-r_square, -l_edge/2.0, -l_edge/2.0))
            .rotate(v3(45., 0., 0.)),
        ]))
}

fn main() {
    let l_edge = 5.;
    let r_square = 2.0_f32.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let r_hexagon = 0.75 * 3.0_f32.powf(0.5) * l_edge;  // height of truncated octahedron between hexagonal faces
    let t = beveled_truncated_octahedron(l_edge);
    let bevel = 1.;
    let piece = t.clone()
        .add_map(move |x| x.translate(v3(2.*r_square, 0., 0.)))
        .add_map(move |x| x.translate(v3(0., 2.*r_square, 0.)))
        .add(t.clone().translate(v3(r_square, r_square, r_square)))
        ;
    let column = t.clone()
        .iter_translate(v3(r_square, r_square, r_square), 5)
        .union()
        .rotate(v3(0., 0., 45.))
        .rotate(v3((-1.0_f32/3.).acos()*180.0/PI/2., 0., 0.))
        .translate(v3(-3.*r_square,0.,0.))
        ;

    println!("{}", &piece.add(column));
    // println!("$fn=64;\n{}", circle_beveled_box(v3(l_edge, 2.*r_square, l_edge), bevel));
}

