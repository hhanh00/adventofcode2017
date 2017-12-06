use std::fs::File;
use std::io::{BufReader, BufRead};

fn max_block(blocks: &Vec<i32>) -> (usize, i32) {
  let (i, b) = blocks.iter().enumerate().max_by_key(|&(index, item)| 100*item-index as i32).unwrap();
  return (i, *b);
}

fn redistribute(blocks: &mut Vec<i32>) {
  let len = blocks.len() as i32;
  let (i, b) = max_block(blocks);
  blocks[i] = 0;
  let b2 = b/len;
  for b in blocks.iter_mut() { 
    *b += b2;
  }
  let r = b-b2*len;
  for _i in 0..r {
    blocks[((i as i32 + _i + 1) % len) as usize] += 1;
  }
}

pub fn d6_main() {
    let f = File::open("d6.txt").expect("Cannot open file");
    let r = BufReader::new(&f);

    let line = r.lines().next().unwrap().unwrap();
    let mut mem: Vec<i32> = line.split_whitespace().map(|w| w.parse().unwrap()).collect();

    println!("{:?}", mem);
    let mem0 = mem.clone();
    let mut mem2 = mem.clone();

    // Turtoise & Hare algorithm for finding loops in linked lists
    for _steps in 0.. {
      redistribute(&mut mem); // turtoise moves one step at a time
      redistribute(&mut mem2); // hare moves two steps at a time
      redistribute(&mut mem2);

      // println!("A {:?}", mem);
      // println!("B {:?}", mem2);

      if mem == mem2 { // when hare meets turtoise again, it has lapped once
        break;
      }
    }

    let mut start = 0;
    let mut length = 0;
    mem = mem0;
    // mem2 does not change. turtoise and hare are now N steps apart where N is a multiple of the loop
    // length
    for steps in 0.. {
      if mem == mem2 {
        start = steps;
        break;
      }
      redistribute(&mut mem); // move turtoise and hare at the same speed
      redistribute(&mut mem2); // until the cycle makes the hare warp back to meet the turtoise, both are then on the 
      // beginning of the cycle
    }

    for steps in 0.. {
      redistribute(&mut mem2); // now move only the hare until it comes back to beginning of the cycle
      if mem == mem2 {
        length = steps + 1;
        break;
      }
    }

    let v = start + length;
    println!("start {}, length {}, total {}", start, length, v);
}