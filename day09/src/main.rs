use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let lines : Vec<String> = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap()).collect();

    
    let mut h = Point::new();
    let mut t = h.clone();
    let mut seen = HashSet::new();
    seen.insert(h.clone());

    let instrs :Vec<(&str,i32)>= lines.iter().map(|l|{
        let (direction, a) = l.split_once(" ").unwrap();
        let dist : i32 = a.parse().unwrap();
        (direction,dist)
    }).collect();

    for (direction,dist) in instrs {
        for _ in 0..dist {
            update_head(direction, &mut h);
            t = updated_tail(h,t);
            seen.insert(t.clone());
            
        }
    }
    println!("part1: {} ",seen.len());
}

fn update_head(direction: &str, h: &mut Point){
    match direction {
        "R" => {
            h.x +=1;
        },
        "U" => {
            h.y +=1;
        },
        "L" => {
            h.x -=1;
        },
        "D" => {
            h.y -=1;
        },
        _ => panic!()
    }
}

fn updated_tail(h: Point, mut t: Point) -> Point{
    let dx = h.x-t.x;
    let dy = h.y-t.y;

    match (dx.abs(),dy.abs()) {
        (0,0) | (1,0) | (0,1) | (1,1) => (),
        (2,0) =>{
            t.x += dx/2;
        },
        (0,2) => {
            t.y += dy/2;
        },
        (2,1) => {
            t.y += dy;
            t.x += dx/2;
        },
        (1,2) => {
            t.x += dx;
            t.y += dy/2;
        },
        _ => todo!()
    }
    t
}

#[derive(Hash, Eq, PartialEq)]
#[derive(Clone, Copy, Debug)]
struct Point{
    x : i32,
    y : i32
}

impl Point {
    fn new() -> Self{
        Self { x:0, y: 0 }
    }
}