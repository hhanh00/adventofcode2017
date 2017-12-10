use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

struct Node<'a> {
    parent: &'a mut str,
    name: &'a str,
    count: i32,
    children: Vec<&'a str>,
}

pub fn main() {
    let f = File::open("d7.txt").expect("Cannot open file");
    let r = BufReader::new(&f);
    let re = Regex::new(r"^(?P<name>\w+) \((?P<count>\d+)\)(?: -> (?P<children>.*))?$").unwrap();
    
    for line in r.lines() {
        let line = line.unwrap();
        println!("{}", line);
        for cap in re.captures_iter(&line) {
            let children: Vec<&str> = cap.name("children").map(|c| c.as_str()).unwrap_or("").split(", ").collect();
            println!("{} {}", &cap["name"], &cap["count"]);
            println!("{:?}", children);
        }
    }

}