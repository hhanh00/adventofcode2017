use std::fs::File;
use std::io::{BufReader, BufRead};
use hex;

const N: i32 = 256;

fn reverse(v: &mut Vec<i32>, pos: i32, length: i32) {
  for i in 0..(length/2) {
    let x = ((pos+i) % N) as usize;
    let y = ((pos+length-i-1) % N) as usize;
    let t = v[x];
    v[x] = v[y];
    v[y] = t;
  }
}

fn hash_round(v: &mut Vec<i32>, pos: &mut i32, skip: &mut i32, steps: &Vec<u8>) {
  for step in steps {
    let s = *step as i32;
    reverse(v, *pos, s);
    *pos = (*pos+s+*skip) % N;
    *skip += 1;
    // println!("{:?}", v);
  }
}

pub fn main() {
  let mut v: Vec<i32> = (0..N).collect();
  let f = File::open("d10.txt").expect("Cannot open file");
  let r = BufReader::new(&f);
  let line = r.lines().next().unwrap().unwrap();
  /*
  let steps: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();

  println!("{:?}", steps);

  println!("{:?}", v);
  */
  let mut pos = 0;
  let mut skip = 0;

  let mut steps2 = line.as_bytes().to_vec();
  steps2.extend_from_slice(&[17, 31, 73, 47, 23]);

  // hash_round(&mut v, &mut pos, &mut skip, &steps);
  // println!("{}", v[0]*v[1]);

  println!("{:?}", steps2);

  for _ in 0..64 {
    hash_round(&mut v, &mut pos, &mut skip, &steps2);
  }

  let mut dense_hash = vec![0; 16];
  for i in 0..16 {
    for j in 0..16 {
      dense_hash[i] ^= v[i*16 + j] as u8;
    } 
  }
  println!("{}", hex::encode(dense_hash));
}
