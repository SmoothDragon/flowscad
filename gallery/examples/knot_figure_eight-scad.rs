use flowscad::*;


fn main() {
    // let points = [(0,2), (0,10), (8,10), (8,4), (4,4), (4,8), (10,8), (10,0), (2,0), (2,6), (6,6), (6,2), (0,2)]
    let points = [v2(0,2), v2(0,10), v2(8,10), v2(8,4), v2(4,4), v2(4,8), v2(10,8), v2(10,0), v2(2,0), v2(2,6), v2(6,6), v2(6,2), v2(0,2)];
    let unit = 8.;
    let vertex = D3::beveled_box(v3(unit, unit, unit), 1);
    let edges = points.pairs()
        .map(|(x,y)| 
             vertex.clone().translate(XYZ::from(x * unit))
             .add(vertex.clone().translate(XYZ::from(y * unit)))
             .hull()
             )
        .union();
    let cutter = D3::cuboid(v3(1.414,unit,1.414))
                .rotate(v3(0,-45,0))
                .translate(v3(unit/2., -unit/2., unit/2.-1.))
                .add_map(|x| x.translate(v3(-unit, 0, 0)))
                // .add_map(|x| x.translate(v3(0,0,-unit)))
                .add_map(|x| x.rotate(v3(180,0,90)))
                .rotate(v3(0,0,90))
                .translate(v3(unit/2., unit/2., unit/2.))
                ;
    let bevels = [v2(2,2), v2(4,6), v2(6,4), v2(8,8)]
    // let bevels = [v2(0,0)]
                .iter()
                .map(|x| cutter.clone().translate(XYZ::from(*x * unit)))
                .union()
                ;

    println!("{}", (edges-bevels).scad());
    // println!("$fn=64;\n{}", circle_beveled_box(v3(l_edge, 2.*r_square, l_edge), bevel));
}

