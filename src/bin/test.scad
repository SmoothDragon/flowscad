$fn=64;hull() {
	circle(r = 5);
	translate(v = [10, 0]) {
		circle(r = 5);
	}
	translate(v = [0, 10]) {
		circle(r = 5);
	}
}

