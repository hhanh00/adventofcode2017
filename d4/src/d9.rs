use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
  let f = File::open("d9.txt").expect("Cannot open file");
  let r = BufReader::new(&f);
  let line = r.lines().next().unwrap().unwrap();

  let mut in_garbage = false;
  let mut skip_next = false;
  let mut level = 0;
  let mut score = 0;
  let mut garbage_count = 0;

  println!("{}", line);
  for c in line.chars() {
    // println!("> {} {} {} {} {}", c, in_garbage, skip_next, level, score);
    match c {
      _ if skip_next => skip_next = false,
      '>' => in_garbage = false,
      '!' => skip_next = true,
      _ if in_garbage => garbage_count += 1,
      '{' => {
        level += 1;
        score += level;
      },
      '}' => level -= 1,
      '<' => in_garbage = true,
      ',' => (),
      _ => panic!("unexpected char"),
    }
    // println!("< {} {} {} {} {}", c, in_garbage, skip_next, level, score);
  }

  println!("{} {}", score, garbage_count);
}

