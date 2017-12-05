use std::fs::File;
use std::io::{BufReader, BufRead};

fn run_program1(program: &mut Vec<i32>) -> i32 {
  let mut pc = 0i32;
  let mut step = 0;
  loop {
    if pc < 0 || pc >= program.len() as i32 {
      return step;
    }
    let upc = pc as usize;
    let offset = program[upc];
    program[upc] += 1;
    pc += offset;
    step += 1;
  }
}

fn run_program2(program: &mut Vec<i32>) -> i32 {
  let mut pc = 0i32;
  let mut step = 0;
  loop {
    if pc < 0 || pc >= program.len() as i32 {
      return step;
    }
    let upc = pc as usize;
    let offset = program[upc];
    program[upc] += if offset >= 3 { -1 } else { 1 };
    pc += offset;
    step += 1;
  }
}

pub fn d5_main() {
    let f = File::open("d5.txt").expect("Cannot open file");
    let r = BufReader::new(&f);

    let mut c: Vec<i32> = r.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let steps = run_program(&mut c);
    println!("{:?}", steps);
}
