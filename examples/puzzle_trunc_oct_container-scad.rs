use flowscad::*;

fn main() {
    let l_edge = 8.;
    let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let gap = 0.1;
    let x = D3::truncated_octahedron(l_edge + gap)
        .add_map(move |x| x.translate(v3(2.0*r_square,0,0)))
        .add_map(move |x| x.translate(v3(0,2.0*r_square,0)))
        .add_map(move |x| x.translate(v3(0,0,2.0*r_square)))
        ;
    let result = (x.clone().translate(v3(2.0*r_square,0,0)) + x.clone().translate(v3(0,2.0*r_square,0)) + x.clone().translate(v3(0,0,2.0*r_square)) + x.clone().translate(v3(r_square, r_square, r_square)))
        .add_map(move |x| x.translate(v3(2.0*r_square,0,0)))
        .add_map(move |x| x.translate(v3(0,2.0*r_square,0)))
        .add_map(move |x| x.translate(v3(0,0,2.0*r_square)))
        // .add(D3::cube(4.*r_square))
        .translate(v3(r_square, 1.*r_square, 1.*r_square))
        ;
    let container = D3::beveled_box(v3(6.*r_square, 6.*r_square, 6.*r_square), 10.*gap)
        .difference(result.clone())
        .rotate(v3(45,-(2.0_f64.powf(-0.5)).atan()*180./PI,0))
        .translate(v3(0,0,-1.5*r_square))
        .intersection(D3::cube(1000).translate(v3(-500,-500,0)))
        // .difference(D3::cuboid(v3(8.*r_square, 7.*r_square, 6.*r_square)).translate(v3(r_square, 1.*r_square, 1.*r_square)))
        // .add(D3::truncated_octahedron(l_edge).translate(v3(r_square, 1.*r_square, 1.*r_square)))
        // .minkowski(D3::truncated_octahedron(gap))
        // .invert(999.)
        // .minkowski(D3::truncated_octahedron(gap))
        // .invert(998.)
        ;

    // println!("{}", result);
    println!("{}", container);
}

