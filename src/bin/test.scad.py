import solid2 as sd

fn =64;
c = sd.circle(5)
piece = sd.hull()(c, sd.translate([10,0])(c), sd.translate([0,10])(c))

final = sd.scad_render(piece, file_header=f'$fn={fn};')
print(final)
