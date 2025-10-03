use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_base = X(36.);
    let l_base = X(275.);
    let h_base = X(42.);
    let d_post = X(25.5);
    let d_hole = X(17.3);
    let gap_holes = X(142.);

    let side_hole = D3::frustum_r(w_base, d_hole/2+1,d_hole/2)
        .rotate_x(90)
        // .translate( (gap_holes/2, 5-w_base/2, h_base-d_hole/2-7) )
        .translate( (gap_holes/2, w_base/2-5, h_base-d_hole/2-7) )
        ;
    let result = D3::cylinder_r(h_base, w_base/2)
        .add_map(|x| x.translate_x(l_base/2))
        .hull()
        .sub(side_hole)
        .add_map(|x| x.mirror( [1,0,0]))
        .sub(D3::cylinder_r(h_base, d_post/2).translate_z(5))
        .rotate_z(39)
        ;
    

    println!("$fn=128;\n{}", &result);
    Ok(())
}
