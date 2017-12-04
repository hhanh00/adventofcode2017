use std::fs::File;
use std::io::{BufReader, BufRead};

fn check(passphrase: &Vec<&str>) -> bool {
    for i in 0..passphrase.len()-1 {
        for j in i+1..passphrase.len() {
            if passphrase[i] == passphrase[j] {
                return false;
            }
        }
    }
    return true;
}

fn check2(passphrase: &Vec<&str>) -> bool {
    for i in 0..passphrase.len()-1 {
        for j in i+1..passphrase.len() {
            let mut x = passphrase[i].to_owned();
            let mut y = passphrase[j].to_owned();
            let a = unsafe { x.as_bytes_mut() };
            let b = unsafe { y.as_bytes_mut() };
            a.sort();
            b.sort();
            if a == b {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let f = File::open("input.txt").expect("Cannot open file");
    let r = BufReader::new(&f);

    let c = r.lines().filter(|ref line| {
        let l = line.as_ref().unwrap();
        let passphrase: Vec<&str> = l.split_whitespace().collect();
        check2(&passphrase)
    }).count();

    println!("{}", c);
}
