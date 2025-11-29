use flowscad::*;


fn main() {
    let square = Face::from(vec![
        C32::new(10.,0.), 
        C32::new(0.,10.), 
        C32::new(-10.,0.), 
        C32::new(0.,-10.),
    ]);
    
    let (xy_min, xy_max) = square.bbox();
    let wh = xy_max - xy_min;
    println!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" width="50mm" height="50mm" viewBox="{} {} {} {} ">
  {}
</svg>"#,
        xy_min.re,
        xy_min.im,
        wh.re,
        wh.im,
        square.svg(),
        );

}

