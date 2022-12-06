use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

fn main() {
    let lines = BufReader::new(File::open("input.txt").expect("open failed"))
        .lines()
        .map(|l|l.unwrap())
        .collect::<Vec<String>>();
    let line = &lines[0].chars().collect::<Vec<char>>();
    println!("part1: {}",find_unique_range(line, 4));
    println!("part2: {}",find_unique_range(line, 14));
}

fn find_unique_range(line : &Vec<char>, of_size : usize) -> usize{
    let mut window : VecDeque<char> = VecDeque::new();

    for i in  0..line.len(){
        let ch = line[i];
        if window.len() < of_size {
            window.push_back(ch);
        }else{
            let mut iniq = HashSet::new();
            if  window.iter().all( |x| iniq.insert(x)) {
                return i;
            }
            else{
                window.pop_front();
                window.push_back(ch);
            }
        }
    }
    panic!()
}