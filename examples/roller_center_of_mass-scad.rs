use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

/* Developable roller design
 * Similar to a oloid, but the distance between the circle centers is r * sqrt(2).
 * This means that the center of mass (COM) has height.
 * From an artistic point of view, this means that a sphere could be positioned at the COM and it
 * would appear fixed while rolling.
 */

fn main() -> Result<()> {
    let gap = 0.1;
    let h_layer = 0.2;
    let r_disc = 50.0;
    let h_disc = 3.0;
    let r_shift = r_disc * 2.0_f32.sqrt();
    let r_ball = r_disc * 0.5;
    
    let base = D3::cylinder_r(h_layer, r_disc)
        .center()
        .add_map(|x| x.translate_x(r_shift).rotate_x(90.0))
        .hull()
        // .and(D3::cylinder_r(h_disc, 10.0 * r_disc).center())
        .and(D3::cuboid([10.0 * r_disc, 10.0 * r_disc, h_disc]).center())
        .sub(D3::cuboid([2.0 * r_disc, h_disc + 2.0 * gap, r_disc]).center()
            .translate_x(r_disc * (1.0 + 2.0_f32.powf(-0.5)) - gap)
            )
        .translate_x(-0.5 * r_shift)
        ;
    let result = base;

    // /// Uncomment for visual check of non overlap
    // let result = result.clone()
        // .add_map(|x| x.rotate([90, 180, 0])
            // )
        // ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

