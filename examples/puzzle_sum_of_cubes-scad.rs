use flowscad::*;

fn main() {
    let l_shape = (D3::cuboid(v3(2, 3, 1)) + D3::cuboid(v3(1,3,3)))
        .iter_translate(v3(0,4,0), 3)
        .union()
        .translate(v3(-3,0,0))
        ;

    let block = D3::cuboid(v3(5,3,2))
        .iter_translate(v3(0,4,0), 3)
        .union();
    let result = (block+l_shape)
        .scale(5);
    println!("{}", &result);
}

