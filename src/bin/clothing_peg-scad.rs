use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let h_cylinder = X(29.);
    let d_cylinder = X(22.3);
    let d_hole = d_cylinder-6;
    let chamfer = X(0.5);
    let d_stem = X(7.8);
    let h_stem = X(12.5);
    let w_stem = X(12.5);
    let d_end = X(19.6);
    let h_end = X(6.);
    let r_fillet = X(2.);
    let w_slot = X(1.8);
    let d_peg = X(4.);
    let peg_chamfer = X(1.5);
    let cut_height = X(100.);
    let base = D3::chamfer_cylinder_d(h_cylinder, d_cylinder, chamfer)
        .translate_z(-h_cylinder/2)
        .add(D3::cylinder_d(w_stem, d_stem)
             .translate( (-(h_stem-d_stem)/2, 0, d_cylinder/2) )
             .add_map(|x| x.translate_x(h_stem-d_stem))
             .hull()
             .rotate_y(90)
             )
        .add(D3::chamfer_cylinder_d(h_end, d_end, chamfer)
             .rotate_y(90)
             .translate_x(d_cylinder/2+w_stem)
             )
        .fillet_radius(r_fillet)
        .difference(D3::cylinder_d(cut_height, d_hole).center())
        .difference(D3::cuboid( (cut_height, w_slot, cut_height) ).center()
                    .translate_x(cut_height/2)
                    )
        .add(D3::chamfer_cylinder_d((d_cylinder+d_hole)/4, d_peg, peg_chamfer)
             .rotate_y(-90)
             )
        ;
    
    println!("$fn=128;\n{}", &base);
    Ok(())
}

