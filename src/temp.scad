minkowski() {
  scale(v = 4) {
    union() {
      scale(v = [3, 2]) {
        translate(v = [4, 5]) {
          union() {
            circle(r = 10);
            circle(r = 4);
          }
        }
      }
      square(size = 9);
    }
  }
  square(size = [0.5, 1.5]);
}
