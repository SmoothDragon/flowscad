use flowscad::*;

fn level_shape(level:u32, total:u32) -> D2 {
    let n = 360;
    let r = X(30.);
    let _h = 10;
    let shift = X(1.)*level/total;
    let points = (0..n)
        .map(|ii| {let theta = PI / 180.0 * ii as f32;
             // v2(r, 0).rotate(theta.0)
             v2(r+shift*(8.0_f32*theta).sin(), 0).rotate(theta)
        })
         .collect::<Vec<XY>>()
         ;
    D2::polygon(points)
}


fn main() {
    let gap = 0.15;  // Gap between plug and adjacent objects

    let id_strut = 5.0;
    let l_strut = 126.4;

    let h_plug = 7.0;
    let d_plug = id_strut - gap;

    // let l_tendon = 0.5 * l_strut + d_plug;
    let l_edge = 70.0;
    let l_tendon = l_edge / 3.0_f32.sqrt();
    let h_tendon = 1.0;
    let w_tendon = 1.2;

    let plug = D3::frustum_d(h_plug, d_plug, 0.9*d_plug)
        .translate_z(h_tendon)
        .add(D3::cylinder_d(h_tendon, d_plug))
        ;
    let plug_half = plug.clone()
        .and(D3::cube(4.0 * h_plug).center().translate_x(-2.0 * h_plug))
        ;

    let tendon = D3::cuboid( v3(l_tendon, w_tendon, h_tendon) )
        .translate_y(-0.5 * w_tendon)
        ;

    let result = tendon.clone()
        .add(plug_half.clone().translate_x(l_tendon))
        .iter_rotate( [0, 0, 120], 3)
        .union()
        .translate_x(-l_tendon)
        .add_map(|x| x.mirror([1,0,0]))
        .add(plug.clone())
        .add_map(|x| x
            .rotate([0,0,-60])
            .translate([1.5*l_tendon, 3.*0.866*l_tendon,0.])
        )
        // .add_map(|x| x.rotate([0,0,180]))
        // .translate_z(h_tendon)
        // .add(D3::cylinder_d(h_tendon, d_plug))
        // .and(D3::cube(4.0 * h_plug).center().translate_x(-2.0 * h_plug - gap / 2.0))
        // .translate_x(l_tendon + gap / 2.0)
        // .add(D3::cuboid( v3(l_tendon, w_tendon, h_tendon) ).translate_y(-0.5 * w_tendon))
        ;

    println!("$fn=100;\n{}", &result);
}
