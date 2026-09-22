use flowscad::*;


fn holding_cube_frame<IL: Into<X>, IW: Into<X>, IB: Into<X>, IG: Into<X>>
    (il_cube: IL, iw_wall: IW, ibevel: IB, igap: IG) -> D3 {
    let bevel = ibevel.into();
    let l = il_cube.into();
    let w = iw_wall.into();
    let gap = igap.into();
    let base = D3::beveled_box( v3(1,1,0)*(l+2*w+2*gap) + v3(0,0,w), bevel).center()
        .translate(-(l/2+w+gap)*v3(1,1,0))
        ;
    let corner = D3::beveled_box( v3(w,2*w,l+2*w+2*gap), bevel)
        .add(D3::beveled_box( v3(w,2*w,l+2*w+2*gap), bevel))
        .add(D3::beveled_box( v3(2*w,w,l+2*w+2*gap), bevel))
        .add(D3::beveled_box( v3(l+2*w+2*gap,w,2*w), bevel))
        .add(D3::beveled_box( v3(l+2*w+2*gap,w,2*w), bevel)
             .translate( v3(0,0,l+2*gap) )
             )
        .add(D3::beveled_box( v3(0.707*(l+2*w+2*gap),w,2*w), bevel)
             .translate( v3(-w,0,-w))
             .rotate((0,-45,0))
             .add_map(move|x| x.mirror( (1,0,0) ))
             .add_map(move|x| x.mirror( (0,0,1) ).translate((0,0,l-w)))
             .translate( v3((l+2*gap)/2+w,0,1.414*w ))
             )
        .translate(-(l/2+w+gap)*v3(1,1,0))
        .iter_rotate((0,0,90),4)
        .union()
        ;
    base+corner
}

fn main() {
    let ll = 11.;
    let bevel = 1.0;
    let wall = 4;
    let gap = 0.2;

    let result = holding_cube_frame(5.0*ll, wall, bevel, gap);
    println!("{}", result);

}


