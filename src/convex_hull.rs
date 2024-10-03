use std::io::{BufRead, Write};
use std::process::{Command, Stdio};
use std::fmt::Write as FmtWrite;

/* QHull input and output description
Generic input for `qhull` is a text file with lines as:
dim:uint
num_vertices:uint
points:[float; dim] * num_vertices

Example input for octahedron:
```
3
12
1 1 0
-1 1 0
-1 -1 0
1 -1 0
1 0 1
-1 0 1
-1 0 -1
1 0 -1
0 1 1
0 -1 1
0 -1 -1
0 1 -1
```

Multiple forms of output may be selected.
This describes the output of `qhull o` which is the most useful for our purposes.

Generic output is lines of text as:
dim: uint
num_vertices:uint num_faces:uint num_edges:uint
vertices:[float; dim] * num_vertices
faces:[num_vertices_on_face + [uint; dim]] * num_faces

Example output for octahedron:
```
3
12 14 24
     1      1      0 
    -1      1      0 
    -1     -1      0 
     1     -1      0 
     1      0      1 
    -1      0      1 
    -1      0     -1 
     1      0     -1 
     0      1      1 
     0     -1      1 
     0     -1     -1 
     0      1     -1 
3 6 11 1 
3 10 6 2 
3 11 7 0 
3 7 10 3 
4 0 7 3 4 
4 7 11 6 10 
4 1 5 2 6 
3 4 8 0 
3 8 5 1 
4 8 1 11 0 
3 9 4 3 
3 5 9 2 
4 4 9 5 8 
4 2 9 3 10 
```
*/

/// Use Qhull to compute convex hull of given points.
/// `qhull` is run as a command line program.
/// The bulk of the work here is creating the format expected and parsing the result.
pub fn convex_hull_3d(points: Vec<[f64; 3]>) -> (Vec<[f64; 3]>, Vec<Vec<u32>>) {
    let points_text = points.iter()
        .map(|v3| format!("{} {} {}\n", v3[0], v3[1], v3[2]))
        .collect::<String>();
    let mut qhull = Command::new("qhull")
        .arg("o")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn qhull process");
    let points_data = format!("3\n{}\n{}\n", points.len(), points_text); 

    let mut stdin = qhull.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(points_data.as_bytes()).expect("Failed to write to stdin");
    });

    let output = qhull.wait_with_output().expect("Failed to read stdout");
    let mut lines = output.stdout.lines();
    let _dim: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let binding = lines.next().unwrap().unwrap();
    let vfe = binding.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let vertices = (0..vfe[0])
        .map(|_| { let binding = lines.next().unwrap().unwrap();
            TryInto::<[f64; 3]>::try_into(
                binding.split_whitespace()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
            ).unwrap()
        }).collect::<Vec<_>>();
    let faces = (0..vfe[1])
        .map(|_| { let binding = lines.next().unwrap().unwrap();
            binding.split_whitespace().skip(1)
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }).collect::<Vec<_>>();
    (vertices, faces)
}

pub fn convex_hull_2d(points: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    let points_text = points.clone().iter()
        .map(|v2| format!("{} {}\n", v2[0], v2[1]))
        .collect::<String>();
        // .fold(String::new(), |mut output, v2| {
            // let _ = write_fmt!(output, "{} {}\n", v2[0], v2[1]);
            // output
        // });

// fn hex_encode(bytes: &[u8]) -> String {
        // bytes.iter().fold(String::new(), |mut output, b| {
                    // let _ = write!(output, "{b:02X}");
                            // output
                                    // })
// }
    let mut qhull = Command::new("qhull")
        .arg("Fx")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn qhull process");
    let points_data = format!("2\n{}\n{}\n", points.len(), points_text); 

    let mut stdin = qhull.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(points_data.as_bytes()).expect("Failed to write to stdin");
    });

    let output = qhull.wait_with_output().expect("Failed to read stdout");
    let mut lines = output.stdout.lines();
    let num_points: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    (0..num_points)
        .map(|_| { let binding = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
            points[binding]
        }).collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convex_hull_2d() {
        assert_eq!(convex_hull_2d(vec![[0.,0.], [2.,0.], [1.,0.], [0.,2.], [2.,2.], [1.,1.]]),
            vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]);
    }
}

