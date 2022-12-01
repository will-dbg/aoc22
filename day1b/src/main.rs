use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::BinaryHeap;

fn main() {
    let f = BufReader::new(File::open("../day1a/input.txt").expect("open failed"));
    let mut pq = BinaryHeap::new();
    let mut curr = 0;
    for line in f.lines().map(|l|l.unwrap()){
        match line.parse::<i32>() {
            Ok(v) => curr = curr+v,
            _ => {pq.push(curr); curr =0}
        }
    }

    let mut sum = 0;
    for _ in 0..3{
        sum = sum + pq.pop().unwrap();
    }
    print!("{}",sum)
}
