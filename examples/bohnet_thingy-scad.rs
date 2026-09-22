use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let od_base = 6.1;
    let id_base = 2.3;
    let h_base = 9.9;
    let h_base_lip = 1.0;
    let od_base_lip = 7.8;
    let d_stem = 3.5;
    let h_stem = 15.0;
    
    let base = D3::cylinder_d(h_base, od_base - 1.0 * h_base_lip)
        .add(D3::cylinder_d(h_base - 0.5 * h_base_lip, od_base)
            .translate_z(0.5 * h_base_lip)
        )
        .hull()
        .add(D3::cylinder_d(h_base_lip, od_base_lip)
            .translate_z(h_base - h_base_lip)
        )
        .sub(D3::cylinder_d(h_base, id_base)
            .center()
        )
        // .hull()
        .add(D3::cylinder_d(h_stem, d_stem)
            .add(D3::cylinder_d(h_stem + 0.4 * d_stem, 0.1 * d_stem))
            .hull()
            .translate_z(h_base)
        )
        .rotate_x(90.0)
        ;
    let result = base;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

