use flowscad::*;

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
    let l_edge = 6.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    // let r_hexagon = 0.75 * 3.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between hexagonal faces
    // let r_square = 10.;
    // let to = D3::truncated_octahedron(l_edge);
    // let gap = 0.1;
    let base_hex = D3::truncated_octahedron(2.2*r_square)
        .rotate(v3(45, -90.+2.0_f64.powf(0.5).atan()*180.0/PI, 30))
        .add_map(move |x| x.rotate(v3(0,0,60)))
        .add_map(move |x| x.rotate(v3(0,0,60)))
        .add_map(move |x| x.rotate(v3(0,0,60)))
        .translate(v3(0,0,4.0*3.0_f64.powf(-0.5)*r_square+3.9))
        ;
    let diag_cube = D3::beveled_box(v3(6.0*r_square,6.0*r_square,6.0*r_square), r_square/2.0)
        .rotate(v3(45, -90.+2.0_f64.powf(0.5).atan()*180.0/PI, 0))
        ;
    let result = diag_cube.clone()
        .add(base_hex)
        .difference(diag_cube.translate(v3(0,0,10)))
        // .map(move |x| x.clone() - x.translate(v3(0,0,10)))
        // .add_map(move |x| x.translate(v3(0,2.0*r_square,0)))
        // .add_map(move |x| x.translate(v3(2.0*r_square,0,0)))
        // .minkowski(D3::truncated_octahedron(gap))
        // .invert(1000.)
        // .minkowski(D3::truncated_octahedron(2.0*gap))
        // .invert(900.)
        // .add_map(move |x| x.translate(v3(0,5.0*r_square,0)))
        // .add_map(move |x| x.translate(v3(5.0*r_square,0,0)))
        // .add_map(move |x| x.translate(v3(10.0*r_square,0,0)))
        ;

    println!("{}", result);
    // println!("$fn=64;\n{}", circle_beveled_box(v3(l_edge, 2.*r_square, l_edge), bevel));
}

