use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
  let f = File::open("d11.txt").expect("Cannot open file");
  let r = BufReader::new(&f);

  let line = r.lines().next().unwrap().unwrap();
  let steps: Vec<&str> = line.split(",").collect();

  let mut x: i32 = 0;
  let mut y: i32 = 0;
  let mut z: i32 = 0;
  let mut max_d = 0;

  for step in steps.iter() {
    match *step {
      "ne" => {
        x += 1; z -= 1;
      },
      "n" => {
        y += 1; z -= 1;
      },
      "nw" => {
        y += 1; x -= 1;
      },
      "sw" => {
        x -= 1; z += 1;
      },
      "s" => {
        y -= 1; z += 1;
      },
      "se" => {
        y -= 1; x += 1;
      },
      _ => panic!("unexpected direction"),
    }
    let d = (x.abs() + y.abs() + z.abs()) / 2;
    if d > max_d { 
      max_d = d;
    }
  }

  let d = (x.abs() + y.abs() + z.abs()) / 2;

  println!("{} {} {} {} {}", max_d, d, x, y, z);
}