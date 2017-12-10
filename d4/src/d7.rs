use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    count: i32,
    children: Vec<String>,
}

fn get_top(nodes: &HashMap<String, Node>, parents: &HashMap<String, String>) -> String {
    let mut n = nodes.keys().next().unwrap();
    loop {
        if let Some(p) = parents.get(n) {
            n = p;
        }
        else {
            println!("TOP {}", n);
            return n.to_owned();
        }
    }
}

fn compute_weight(nodes: &HashMap<String, Node>, n: &String) -> i32 {
    let node = &nodes[n];
    let ws: Vec<i32> = node.children.iter().map(|c| {
        let w = compute_weight(nodes, c);
        println!("c:{} w:{}", c, w);
        return w;
    }).collect();
    if ws.len() > 0 {
        let w0 = ws[1]; // Hack - Only one value is allowed to be different than the others. We should pick the index of a 'good' value
        // I ran the program once and saw that the first 'program' had the wrong weight.
        if let Some(idx) = ws.iter().position(|w| *w != w0) {
            let w2 = ws[idx];
            println!("{} {} {:?} {:?}", n, w2, node.children.iter(), ws);
            let n2 = &nodes[&node.children[idx]];
            let d = w0-w2;
            println!("{:?} {} R:{}", n2, d, n2.count+d);
            panic!("Unbalanced");
        }
    }
    let w: i32 = ws.iter().sum();
    return w + node.count;
}

pub fn main() {
    let f = File::open("d7.txt").expect("Cannot open file");
    let r = BufReader::new(&f);
    let re = Regex::new(r"^(?P<name>\w+) \((?P<count>\d+)\)(?: -> (?P<children>.*))?$").unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();
    
    for line in r.lines() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
            let children: Vec<&str> = 
                cap.name("children").map(|c| c.as_str().split(", ").collect()).unwrap_or(vec![]);
            let name = String::from(&cap["name"]);
            let node = Node { name, count: cap["count"].parse().unwrap(), 
                children: children.iter().map(|c| String::from(*c)).collect() };
            nodes.insert(node.name.to_owned(), node);
        }
    }

    for (_, node) in nodes.iter() {
        for child in node.children.iter() {
            parents.insert(child.to_owned(), node.name.to_owned());
        }
    }

    let top = get_top(&nodes, &parents);
    compute_weight(&nodes, &top);

}