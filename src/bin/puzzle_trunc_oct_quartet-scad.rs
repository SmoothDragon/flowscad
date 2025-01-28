use flowscad::*;

fn main() {
    let l_edge = 8.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let gap = 0.1;
    let result = D3::truncated_octahedron(l_edge + gap)
        .add_map(move |x| x.translate(v3(2.0*r_square,0,0)))
        .add_map(move |x| x.translate(v3(0,2.0*r_square,0)))
        .invert(100.)
        .minkowski(D3::truncated_octahedron(2.0*gap))
        .invert(99.)
        // .add_map(move |x| x.translate(v3(4.0*r_square,0,0)))
        // .add_map(move |x| x.translate(v3(0,4.0*r_square,0)))
        ;
    println!("{}", result);
}

