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
    let ll = 7.;
    let bevel = ll/10.;
    let block = D3::beveled_box(v3(ll, ll, ll), bevel);
    let tl = 7.;
    let piece1 = D3::beveled_cube_block( (4, 4, 4), ll, bevel, 0.1);
    let piece2 = D3::beveled_cube_block( (4, 4, 2), ll, bevel, 0.1)
        .translate(v3(5.0*tl,0.,0.))
        ;
    let piece3 = D3::beveled_cube_block( (2, 5, 5), ll, bevel, 0.1)
        .add(D3::beveled_cube_block( (3, 1, 5), ll, bevel, 0.1))
        .translate(v3(0., 5.0*tl, 0.))
        ;
    let piece4 = D3::beveled_cube_block( (2, 5, 1), ll, bevel, 0.1)
        .add(D3::beveled_cube_block( (3, 1, 1), ll, bevel, 0.1))
        .translate(v3(4.*tl, 5.0*tl, 0.))
        ;
    let piece5 = D3::beveled_cube_block( (3, 5, 1), ll, bevel, 0.1)
        .translate(v3(8.*tl, 5.0*tl, 0.))
        ;
    let piece6 = D3::beveled_cube_block( (3, 4, 1), ll, bevel, 0.1)
        .translate(v3(10.*tl, 0., 0.))
        ;
    let piece7 = D3::beveled_cube_block( (2, 2, 3), ll, bevel, 0.1)
        .add(D3::beveled_cube_block( (3, 1, 3), ll, bevel, 0.1))
        .translate(v3(0.*tl, 11.0*tl, 0.))
        ;
    let piece8 = D3::beveled_cube_block( (1, 2, 3), ll, bevel, 0.1)
        .add(D3::beveled_cube_block( (3, 1, 3), ll, bevel, 0.1))
        .translate(v3(4.*tl, 11.0*tl, 0.))
        ;

    let result = piece1
        .add(piece2)
        .add(piece3)
        .add(piece4)
        .add(piece5)
        .add(piece6)
        .add(piece7)
        .add(piece8)
        ;
    let result = D3::rounded_cube(50);
    println!("{}", result);
}

