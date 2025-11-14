use std::fs;
use std::f32::consts::PI;

/// Generate points of the Archimedean spiral r = a + b*theta
fn generate_spiral(a: f32, b: f32, turns: f32, samples_per_turn: usize) -> Vec<(f32, f32)> {
    let total_theta = 2.0 * PI * turns;
    let n = (turns * samples_per_turn as f32).ceil() as usize;
    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = (i as f32) / (n as f32);
        let theta = t * total_theta;
        let r = a + b * theta;
        let x = r * theta.cos();
        let y = r * theta.sin();
        pts.push((x, y));
    }
    pts
}

fn generate_cut_lines(a: f32, b: f32, turns: f32, samples_per_turn: usize) -> Vec<((f32, f32), (f32, f32))> {
    let total_theta = 2.0 * PI * turns;
    let n = (turns * samples_per_turn as f32).ceil() as usize;
    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = (i as f32) / (n as f32);
        let bump = (i%2) as f32 * PI;
        let theta = t * total_theta;
        let r = a + b * theta;
        let x0 = (r+bump) * theta.cos();
        let y0 = (r+bump) * theta.sin();
        let x1 = (r+2.0*PI+bump) * theta.cos();
        let y1 = (r+2.0*PI+bump) * theta.sin();
        pts.push(((x0, y0), (x1, y1)));
        let x0 = (r+3.0*PI+bump) * theta.cos();
        let y0 = (r+3.0*PI+bump) * theta.sin();
        let x1 = (r+5.0*PI+bump) * theta.cos();
        let y1 = (r+5.0*PI+bump) * theta.sin();
        pts.push(((x0, y0), (x1, y1)));
    }
    pts
}

fn generate_spaced_lines(a: f32, b: f32, turns: f32, samples_per_turn: usize) -> Vec<((f32, f32), (f32, f32))> {
    let total_theta = 2.0 * PI * turns;
    let mut theta = 0.0 * PI;
    let mut r = a;  // Initial radius
    let gap = 1.5;  // mm between close part of cut
    let mut i = 0_u32;
    let mut lines = Vec::<((f32, f32), (f32, f32))>::new();
    while theta < total_theta {
        let bump = (i%2) as f32 * PI * 2.0;
        let x0 = (r+bump) * theta.cos();
        let y0 = (r+bump) * theta.sin();
        let x1 = (r+4.0*PI+bump) * theta.cos();
        let y1 = (r+4.0*PI+bump) * theta.sin();
        lines.push(((x0, y0), (x1, y1)));
        // let x0 = (r+3.0*PI+bump) * theta.cos();
        // let y0 = (r+3.0*PI+bump) * theta.sin();
        // let x1 = (r+5.0*PI+bump) * theta.cos();
        // let y1 = (r+5.0*PI+bump) * theta.sin();
        // lines.push(((x0, y0), (x1, y1)));
        i ^= 1;
        let limit = 4.*a;
        if r > limit {
            theta += gap/r;
        } else {
            theta += (1.-0.5*(limit-r)/(limit))*gap/r;
        }
        r = a + b * theta;
    }
    lines
}

/// Produce an SVG path string from points, optionally with decimal formatting
fn path_from_points(points: &[(f32, f32)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut s = String::new();
    // Move to first point
    s += &format!("M{:.3},{:.3}", points[0].0, points[0].1);
    // Line to remaining points
    for &(x, y) in points.iter().skip(1) {
        s += &format!(" L{:.3},{:.3}", x, y);
    }
    s
}

/// Produce a SVG line strings from points, optionally with decimal formatting
fn lines_from_points(points: &[((f32, f32),(f32, f32))]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut s = String::new();
    // Line between point pairs
    for &((x0, y0), (x1, y1)) in points.iter().skip(1) {
        s += &format!(r#"<line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="black" stroke-width="0.5"/>"#, x0, y0, x1, y1);
        s += &"\n";
    }
    s
}

fn main() -> std::io::Result<()> {
    // --- user params ---
    let a = 10.0;               // starting radius
    let b = 3.0;               // radial growth per radian (scale)
    let turns = 6.0;           // number of turns
    let samples_per_turn = 600; // higher = smoother
    let stroke_width = 1.5;
    let svg_filename = "spiral.svg";

    // generate points
    let pts = generate_spiral(a, b, turns, samples_per_turn);
    // let lines = generate_cut_lines(a, b, turns-1.0, samples_per_turn/10);
    let lines = generate_spaced_lines(a, b, turns-1.0, samples_per_turn/10);

    // compute bounding box to center and set viewBox
    let (min_x, max_x) = pts.iter().map(|(x, _)| *x).fold((f32::INFINITY, f32::NEG_INFINITY), |(mn, mx), x| (mn.min(x), mx.max(x)));
    let (min_y, max_y) = pts.iter().map(|(_, y)| *y).fold((f32::INFINITY, f32::NEG_INFINITY), |(mn, mx), y| (mn.min(y), mx.max(y)));

    // add margin
    let margin = 0.0; // in SVG user units
    let width = (max_x - min_x) + 2.0 * margin;
    let height = (max_y - min_y) + 2.0 * margin;

    // shift points so the bounding box sits inside the viewBox with margin
    let dx = -min_x + margin;
    let dy = -min_y + margin;
    let shifted: Vec<(f32, f32)> = pts.iter().map(|(x, y)| (x + dx, y + dy)).collect();

    // build path data
    // let path_data = path_from_points(&shifted);
    let path_data = path_from_points(&pts);
    let line_data = lines_from_points(&lines);

    // Build SVG content
    let svg_path = format!(
        r#"<path d="{path}" fill="none" stroke="{action}" stroke-width="{sw}" stroke-linecap="round" stroke-linejoin="round"/>
        "#, 
        path = path_data,
        sw = stroke_width,
        action = &"red",
    );

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{min_x} {min_y} {w:.3} {h:.3}" width="{w:.3}mm" height="{h:.3}mm">
            <rect width="100%" height="100%" x="{min_x}" y="{min_y}" fill="white"/>
            {svg_path}
            {lines}
        </svg>
"#,
        w = width,
        h = height,
        svg_path = svg_path,
        min_x = min_x,
        min_y = min_y,
        lines = line_data,
        // path = path_data,
        // sw = stroke_width
    );
    fs::write(svg_filename, svg)?;
    println!("Wrote {}", svg_filename);
    Ok(())
}

