use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let od_leg = X(51.25);
    let id_leg = X(48.25);
    let od_insert = X(47.25);
    let id_insert = X(42.5);
    let w_cross = X(2.5);
    let h_cross = X(15.0);
    let od_bottom = X(58.5);
    let h_bottom = X(25.0);
    let gap = X(0.2);
    let h_extension = X(50.);
    let h_base_layer = X(5.);
    let bevel = X(1.);
    let h_join = h_extension-h_bottom+h_base_layer;

    // To upper part should receive the metal leg in a stable way
    let upper = D3::chamfer_cylinder_d(h_bottom, od_bottom, bevel)
        .difference(D3::chamfer_cylinder_d(h_bottom, od_leg+gap, bevel)
                    .translate_z(h_base_layer))
        .add(D3::chamfer_cylinder_d(h_bottom, id_leg-gap, bevel))
        .translate_z(h_join-h_base_layer)
        ;
    // The lower part should connect to the existing foot
    let lower = D3::chamfer_cylinder_d(h_bottom, od_bottom, bevel)
        .difference(D3::chamfer_cylinder_d(h_bottom, od_insert+2*gap, bevel)
                    .translate_z(-h_base_layer))
        .add(D3::chamfer_cylinder_d(h_bottom, id_insert-2*gap, bevel)
             .difference(D3::cuboid( (od_leg, w_cross+2*gap, 2*h_cross ))
                         .center()
                         .add_map(|x| x.rotate_z(90))
                        )
             )
        .translate_z(-h_bottom+h_base_layer)
        ;
    let join = D3::cylinder_d(h_join, od_bottom);

    let result = (upper+lower+join)
        // .difference(D3::cube(200).translate_z(-100))
        ;
    // let h_bottom = X(10.0);
    // let fake_lower = D3::chamfer_cylinder_d(h_bottom, od_bottom, bevel)
        // .difference(D3::chamfer_cylinder_d(h_bottom, od_insert+gap, bevel)
                    // .translate_z(-5))
        // .add(D3::chamfer_cylinder_d(h_bottom, id_insert-gap, bevel)
             // .difference(D3::cuboid( (od_leg, w_cross+gap, 2*h_cross ))
                         // .center()
                         // .add_map(|x| x.rotate_z(90))
                        // )
             // )
        // .translate_z(-h_bottom)
        // ;
    // let result = fake_lower;

    println!("$fn=256;\n{}", &result);
    Ok(())
}

