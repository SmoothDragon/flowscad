use flowscad::*;


// pub fn circle_beveled_box(xyz: Real3, bevel: f64) -> D3 {
    // let x = xyz.0; 
    // let y = xyz.1;
    // let z = xyz.2;
    // let rod = D3::cylinder(x.max(y).max(z), bevel);
    // D3::cuboid(x,y,z) - rod
// }


fn main() {
    let l_edge = 8.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    // let r_hexagon = 0.75 * 3.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between hexagonal faces
    // let r_square = 10.;
    // let to = D3::truncated_octahedron(l_edge);
    let gap = 0.1;
    let to_gap = D3::truncated_octahedron(gap);
    let result = D3::truncated_octahedron(l_edge + gap)
        .add_map(move |x| x.translate(v3(0,2.0*r_square,0)))
        .add_map(move |x| x.translate(v3(2.0*r_square,0,0)))
        // .minkowski(D3::truncated_octahedron(gap))
        .invert(1000.)
        .minkowski(D3::truncated_octahedron(2.0*gap))
        .invert(900.)
        // .add_map(move |x| x.translate(v3(0,5.0*r_square,0)))
        // .add_map(move |x| x.translate(v3(5.0*r_square,0,0)))
        // .add_map(move |x| x.translate(v3(10.0*r_square,0,0)))
        .rotate(v3(0,0,45))
        ;

    println!("{}", result);
    // println!("$fn=64;\n{}", circle_beveled_box(v3(l_edge, 2.*r_square, l_edge), bevel));
}

