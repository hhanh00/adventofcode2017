use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

enum Cond { EQ, NE, LT, LE, GT, GE }

struct Instruction {
  reg: String,
  val: i32,
  ifreg: String,
  cond: Cond,
  cond_val: i32,
}

fn eval(registers: &mut HashMap<String, i32>, instruction: &Instruction) {
  let ifreg_val = registers[&instruction.ifreg];
  let cond_ok = match instruction.cond {
    Cond::EQ => ifreg_val == instruction.cond_val,
    Cond::NE => ifreg_val != instruction.cond_val,
    Cond::LT => ifreg_val < instruction.cond_val,
    Cond::LE => ifreg_val <= instruction.cond_val,
    Cond::GT => ifreg_val > instruction.cond_val,
    Cond::GE => ifreg_val >= instruction.cond_val,
  };
  if cond_ok {
    let r = registers.get_mut(&instruction.reg).unwrap();
    *r += instruction.val;
  }
}

pub fn main() {
    let f = File::open("d8.txt").expect("Cannot open file");
    let r = BufReader::new(&f);
    let re = Regex::new(r"^(?P<reg>\w+) (?P<op>inc|dec) (?P<val>-?\d+) if (?P<ifreg>\w+) (?P<cond>>=|!=|==|>|<|<=) (?P<condval>-?\d+)$").unwrap();
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut alltime_max = 0;

    for line in r.lines() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
          let reg = cap.name("reg").unwrap().as_str().to_owned();
          let op = cap.name("op").unwrap().as_str().to_owned();
          let val_: i32 = cap.name("val").unwrap().as_str().parse().unwrap();
          let val = match op.as_ref() {
            "inc" => val_,
            "dec" => -val_,
            _ => panic!("unexpected operation"),
          };

          let ifreg = cap.name("ifreg").unwrap().as_str().to_owned();
          let cond_str = cap.name("cond").unwrap().as_str().to_owned();
          let cond_val: i32 = cap.name("condval").unwrap().as_str().parse().unwrap();
          let cond = match cond_str.as_ref() {
            "<" => Cond::LT,
            "<=" => Cond::LE,
            ">" => Cond::GT,
            ">=" => Cond::GE,
            "==" => Cond::EQ,
            "!=" => Cond::NE,
            _ => panic!("unknown condition")
          };
          registers.entry(reg.to_owned()).or_insert(0);
          registers.entry(ifreg.to_owned()).or_insert(0);

          let instruction = Instruction { reg, val, ifreg, cond, cond_val };

          eval(&mut registers, &instruction);
          let max = *registers.iter().max_by_key(|&(_, v)| v).unwrap().1;
          alltime_max = if max > alltime_max { max } else { alltime_max };
        }
    }

    let final_max = registers.iter().max_by_key(|&(_, v)| v).unwrap();
    println!("{} {}", alltime_max, final_max.1);

}