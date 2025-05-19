use std::fs;
use flowscad::*;


fn main() {
    let l_cube = 81.0/6.0;
    let bevel = l_cube*0.1;
    let gap = 0.2;
    let piece = vec![
    D3::beveled_cube_block( (4, 4, 3), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 4, 4), l_cube, bevel, gap))
        .add(D3::beveled_cube_block( (4, 2, 4), l_cube, bevel, gap))
        ,
    D3::beveled_cube_block( (2, 4, 2), l_cube, bevel, gap).translate(v3(l_cube,0.,0.))
        .add(D3::beveled_cube_block( (3, 3, 2), l_cube, bevel, gap))
        .add(D3::beveled_cube_block( (1, 3, 3), l_cube, bevel, gap))
        .add(D3::beveled_cube_block( (3, 2, 5), l_cube, bevel, gap))
        .translate(v3(0., 5.0*l_cube, 0.))
        
        ,
    D3::beveled_cube_block( (5, 1, 2), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 3, 2), l_cube, bevel, gap))
        .translate(v3(5.*l_cube, 0.0*l_cube, 0.))
        ,
    D3::beveled_cube_block( (3, 3, 3), l_cube, bevel, gap)
        .translate(v3(11.*l_cube, 0.0*l_cube, 0.))
        ,
    D3::beveled_cube_block( (4, 2, 3), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (2, 3, 3), l_cube, bevel, gap))
        .translate(v3(5.*l_cube, 5.0*l_cube, 0.))
        ,
    D3::beveled_cube_block( (1, 2, 1), l_cube, bevel, gap)
        .translate(v3(3.5*l_cube, 5.0*l_cube, 0.))
        ,
    D3::beveled_cube_block( (2, 2, 2), l_cube, bevel, gap)
        .translate(v3(8.5*l_cube, 2.*l_cube, 0.))
        ,
    D3::beveled_cube_block( (3, 2, 1), l_cube, bevel, gap)
        .translate(v3(0.*l_cube, 10.0*l_cube, 0.))
        ,
    D3::beveled_cube_block( (3, 3, 2), l_cube, bevel, gap)
        .translate(v3(11.*l_cube, 4.0*l_cube, 0.))
        ,
        /*
        */
    ];

    // let result = D3::Union(Box::new(piece.clone()));
    let result = piece.clone().into_iter().union();
    println!("{}", result);

    // for (ii, item) in piece.iter().enumerate() {
        // fs::write(format!("puzzle_prenoam_6x6x6_{ii}.scad"), format!("{}", item)).expect("Unable to write file");
    // }
}


