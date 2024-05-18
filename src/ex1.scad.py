import solid2 as sd

final = sd.circle(r=1)
final += sd.square(2,3, center=True)
final += sd.square(3)
final = sd.translate([3,4])(final)
final = sd.scale([3,4])(final)
final += sd.square(3)
final = sd.scale(3)(final)
final = sd.minkowski()(final, sd.square(15), sd.circle(5))
print(sd.scad_render(final))
