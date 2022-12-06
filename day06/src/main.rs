use std::io::{BufRead, BufReader};
use std::fs::File;
use itertools::structs::Unique;

fn main() {
    let lines = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines().map(|l|l.unwrap()).collect::<Vec<String>>();
    let line = &lines[0].chars().collect::<Vec<char>>();
    let mut window : Vec<char> = Vec::new();

    for i in (0..line.len()){
        let ch = line[i];
        if window.len() < 4 {
            window.push(ch);
        }else{
            window.dedup();
            if window.len() == 4 {
                println!("Part1: {}",i);
                break;
            }
        }
    }
}
