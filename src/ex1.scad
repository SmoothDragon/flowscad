minkowski() {
	scale(v = 3) {
		union() {
			scale(v = [3, 4]) {
				translate(v = [3, 4]) {
					union() {
						circle(r = 1);
						square(center = true, size = [2, 3]);
						square(size = 3);
					}
				}
			}
			square(size = 3);
		}
	}
	square(size = .1);
}

