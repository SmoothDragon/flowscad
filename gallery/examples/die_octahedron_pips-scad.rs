use flowscad::*;
use anyhow::Result;

fn pips(num: u32, d_pip:f32, ir_square:f32) -> D3 {
    let ball = D3::sphere_d(d_pip);
    let s = ir_square;
    match num {
        1 => ball,
        2 => [(s,s),(-s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        3 => [(0.,0.),(s,s),(-s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        4 => [(s,s),(-s,s),(-s,-s),(s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        5 => [(0.,0.),(s,s),(-s,s),(-s,-s),(s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        6 => [(s,0.),(s,s),(-s,s),(-s,0.),(-s,-s),(s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        7 => [(0.,0.),(s,0.),(s,s),(-s,s),(-s,0.),(-s,-s),(s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        8 => [(s,0.),(s,s),(0.,s),(-s,s),(-s,0.),(-s,-s),(0.,-s),(s,-s)].into_iter().map(|(x,y)| ball.clone().translate((x,y,0.0))).union(),
        _ => ball
    }
}


fn main() -> Result<()> {
    let r_oct = 10.;
    let r_face = 0.5*r_oct;
    let pip_shift = r_face*1.17;
    let d_pip = r_face*0.35;
    let s_pip = r_face*0.4;
    let base = D3::octahedron(r_oct)
        .intersection(D3::sphere_r(0.707*r_oct))  // corner edges between faces
        .rotate_z(45)
        .rotate_y(109.47/2.0)
        ;
    let side1 = pips(1, d_pip, s_pip)
        .translate_z(pip_shift)
        .add(pips(8, d_pip, s_pip)
            .translate_z(-pip_shift)
        )
        .rotate_y(-180.0+109.47)
        .render()
        ;
    let side2 = pips(2, d_pip, s_pip)
        .translate_z(pip_shift)
        .add(pips(7, d_pip, s_pip)
            .translate_z(-pip_shift)
        )
        .rotate( (0, -180.0+109.47, 120) )
        .render()
        ;
    let side3 = pips(3, d_pip, s_pip)
        .translate_z(pip_shift)
        .add(pips(6, d_pip, s_pip)
            .translate_z(-pip_shift)
        )
        .rotate( (0, -180.0+109.47, 240) )
        .render()
        ;
    let side4 = pips(4, d_pip, s_pip)
        .translate_z(pip_shift)
        .add(pips(5, d_pip, s_pip)
            .translate_z(-pip_shift)
        )
        .render()
        ;


    let result = (base - (side1 +side2 + side3 + side4))
        .rotate_x(180)
        ;
    // let result = side1;
    println!("$fn=64;\n{}", &result);
    // println!("$fs=0.1;\n{}", &result);
    // println!("$fs=0.2;\n$fa=2;\n{}", &result);
    Ok(())
}
