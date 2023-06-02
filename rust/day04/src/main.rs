use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let lines = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap())
    .collect::<Vec<String>>();

    let x = lines.iter().fold((0,0), |acc,line|
        {
            let r = get_range(line); 
            let mut p1 = acc.0;
            let mut p2 = acc.1;
            if either_contains(&r.0,&r.1){
                p1 = p1 +1;
            }
            if overlap(&r.0,&r.1){
                p2 = p2 +1;
            }
            return (p1,p2);
        }
    );
    println!("part1: {}",x.0);
    println!("part2: {}",x.1);
}

struct Assignment{
    start : usize,
    end : usize
}

impl Assignment {

    fn new(input : &str) -> Self{
        let i : Vec<&str> = input.split('-').collect();
        Self {
            start : i[0].parse().unwrap(),
            end : i[1].parse().unwrap()
        }
    }

    fn contains(&self, other: &Assignment) -> bool {
        return self.start <= other.start && self.end >= other.end
    }

    fn within(&self, b: &Assignment) -> bool {
        self.start >= b.start && self.start <= b.end
    }
}

fn either_contains(a: &Assignment,b: &Assignment)-> bool{
    return a.contains(&b) || b.contains(&a)
}

fn overlap(a: &Assignment, b: &Assignment) -> bool {
    return a.within(b) || b.within(a)
}

fn get_range(str : &str) -> (Assignment,Assignment){
    let i : Vec<&str> = str.split(',').collect();
    return (Assignment::new(i[0]),Assignment::new(i[1]));
}
