use std::fs;
use flowscad::*;


fn main() {
    let h_rise = 70.;
    let d_foot = 30.;
    let h_lip = 4.;
    let w_lip = 4.0;

    let result = D3::cylinder_d(h_rise+h_lip, d_foot+2.*w_lip)
        .sub(D3::cylinder_d(h_rise, d_foot).translate_z(h_rise))
        ;
       
    println!("$fn=256; {}", result);
}


