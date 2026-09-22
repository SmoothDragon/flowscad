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
    let w_disc_rim = 8.0;
    let h_disc = 5.0;
    let r_shift = r_disc * 2.0_f32.sqrt();
    let r_ball = r_disc * 0.5;
    let w_peg = 5.;  // Width of peg hole attaching two halves
    let gap_step = 0.12;  // Width of peg hole attaching two halves
    
    let base = D3::cylinder_r(h_layer, r_disc)
        .center()
        .add_map(|x| x.translate_x(r_shift).rotate_x(90.0))
        .hull()
        .and(D3::cylinder_r(h_disc, r_disc).center())
        .sub(D3::cylinder_r(2.0 * h_disc, r_disc-w_disc_rim).center())
        .add(D3::cuboid([r_disc, 2.0*w_disc_rim + h_disc, h_disc]).center()
            .translate_x(0.65 * r_shift)
            )
        // .and(D3::cuboid([10.0 * r_disc, 10.0 * r_disc, h_disc]).center())
        .sub(D3::cuboid([2.0 * r_disc, h_disc + 0.5 * gap, r_disc]).center()
            .translate_x(r_disc * (1.0 + 2.0_f32.powf(-0.5)) - gap)
            )
        .and(D3::cylinder_r(h_disc, r_disc).center())
        .translate_x(-0.5 * r_shift)
        .add(D3::sphere_r(0.707 * r_disc))
        .left()
        .sub(D3::cuboid(v3(2,1,1) * w_peg).center())
        .rotate(v3(0,90,0))
        ;
    let range = 3;
    let pegs = (-range..range).map(|ii| {
        let xx: f32 = ii as f32;
        D3::beveled_box(v3(2,1,1) * (w_peg - 2.*gap) + v3(2,1,1) * xx * gap_step, 0.5)
        // D3::beveled_box(v3(2. * (1.-gap-xx*gap_step),1,1) * w_peg, 0.5)
            .translate(v3(0.8*r_disc, 2.*xx*w_peg,0))
            // .translate(v3(2. * (1.-gap-xx*gap_step),1,1) * -0.5 * w_peg)
    })
        .union()
        ;
    let result = base.clone().translate_x(1.75*r_disc) + base + pegs;

    // /// Uncomment for visual check of non overlap
    // let result = result.clone()
        // .add_map(|x| x.rotate([90, 180, 0])
            // )
        // ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

