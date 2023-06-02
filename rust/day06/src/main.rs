use std::io::{BufRead, BufReader};
use std::fs::File;

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
    return of_size + line
        .windows(of_size)
        .position(|window| !(0..(of_size-1)).any(|c| (c+ 1..of_size).any(|d| window[c]==window[d]))).unwrap();
}