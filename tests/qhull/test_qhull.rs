use qhull::Qh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qh = Qh::builder().compute(true).build_from_iter([
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, 1.0],
        [0.25, 0.25],
    ])?;

    assert_eq!(qh.num_faces(), 3);

	for simplex in qh.simplices() {
		println!("{:?}", simplex.vertices().unwrap());
	}

    for simplex in qh.simplices() {
        let vertices = simplex
            .vertices()
            .unwrap()
            .iter()
            .map(|v| v.id())
            .collect::<Vec<_>>();

        println!("{:?}", vertices);
    }

    Ok(())
}
