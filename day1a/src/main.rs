use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;

fn main() {
    let mut f = BufReader::new(File::open("input.txt").expect("open failed"));
    let mut max = 0;
    let mut curr = 0;
    for line in f.lines().map(|l|l.unwrap()){
        match line.parse::<i32>() {
            Ok(v) => curr = curr+v,
            _ => {max = cmp::max(curr,max); curr =0}
        }
    }
    max = cmp::max(curr,max);
    print!("{}",max)
}
