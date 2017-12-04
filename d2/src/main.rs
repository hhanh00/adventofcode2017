use std::io::{BufReader, BufRead};
use std::fs::File;

fn checksum_row(line: &String) -> u32 {
    let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let min_row = row.iter().min().unwrap();
    let max_row = row.iter().max().unwrap();
    return max_row-min_row;
}

fn checksum_row2(line: &String) -> Option<u32> {
    let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    for n in row.iter() {
        for d in row.iter() {
            if (n > d) && n % d == 0 {
                return Some(n / d);
            }
        }
    }
    return None
}


fn main() {
    let f = File::open("input.txt").expect("Cannot open file");
    let r = BufReader::new(&f);
    let checksum: u32 = r.lines().map(|line| checksum_row2(&(line.unwrap())).unwrap()).sum();
    println!("{}", checksum);
}
