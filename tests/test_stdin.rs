use std::io::{BufRead, Write};
use std::process::{Command, Stdio};

// use flowscad::*;
use anyhow::Result;


static OCTAHEDRON: &str = "3\n12\n\
1 1 0\n\
-1 1 0\n\
-1 -1 0\n\
1 -1 0\n\
1 0 1\n\
-1 0 1\n\
-1 0 -1\n\
1 0 -1\n\
0 1 1\n\
0 -1 1\n\
0 -1 -1\n\
0 1 -1\n";


fn main() -> Result<()> {
    let mut child = Command::new("qhull")
        .arg("o")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(OCTAHEDRON.as_bytes()).expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    let mut lines = output.stdout.lines();
    // let dim: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let _dim: u32 = lines.next().unwrap()?.parse::<u32>()?;
    let binding = lines.next().unwrap()?;
    let vfe = binding.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let vertices = (0..vfe[0])
        .map(|_| { let binding = lines.next().unwrap().unwrap();
            TryInto::<[f32; 3]>::try_into(
                binding.split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
            ).unwrap()
        }).collect::<Vec<_>>();
    let faces = (0..vfe[1])
        .map(|_| { let binding = lines.next().unwrap().unwrap();
            binding.split_whitespace().skip(1)
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }).collect::<Vec<_>>();
    println!("{:?}", vertices);
    println!("{:?}", faces);
    Ok(())
}
