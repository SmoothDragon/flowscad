use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let s_cube = X(30.);
    let w_cube = X(9.);
    let s_hole = s_cube - 2*w_cube;

    let cube = D3::cube(s_cube).center()
        .sub(D3::cuboid( (s_hole, s_hole, 2*s_cube) )
            .center()
            .add_map(|x| x.clone().rotate_x(90) + x.rotate_y(90))
        )
        .add_map(|x| x.translate( (w_cube+1,w_cube+1,w_cube+1) ))
        ;

    // let end_result = hex + lid.translate_y(2.1*ir_hex);
    let end_result = cube;
    println!("$fn=64;\n{}", &end_result);
    Ok(())
}
