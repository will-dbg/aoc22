use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let lines : Vec<String> = BufReader::new(File::open("input.txt").expect("open failed"))
        .lines()
        .map(|l|l.unwrap()).collect();

    let mut forest :Vec<Vec<u32>> = Vec::new();
    let mut trees : HashSet<(usize,usize)> = HashSet::new();


    for line in lines.iter(){
        let mut vec : Vec<u32> = Vec::new();
        for n in line.chars().map(|c|c.to_digit(10).unwrap()){
            vec.push(n)
        }
        forest.push(vec)
    }


    let s = forest.len();
    for x in 0..s {
        trees.insert((x,0));
        trees.insert((0,x));
        trees.insert((x,s-1));
        trees.insert((s-1,x));
        if x > 0 && x < s-1 {
            
            let mut n = forest[0][x];
            let mut e = forest[x][0];
            let mut r = forest[x][s-1];
            let mut w = forest[s-1][x];
            for y in 0..(s) {
                let nn = forest[y][x];
                let ee = forest[x][y];
                let rr = forest[x][s-1-y];
                let ww = forest[s-1-y][x];
                if nn > n{
                    n = nn;
                    trees.insert((y,x));
                }
                if ee > e {
                    e=ee;
                    trees.insert((x,y));
                }
                if rr > r  {
                    r = rr;
                    trees.insert((x,s-y-1));
                }
                if ww > w {
                    w = ww;
                    trees.insert((s-y-1,x));
                }
            }
        }
    }

    for x in 0..s {
        for y in 0..s {
            if trees.contains(&(x,y)){
                print!("\x1b[93m{}\x1b[0m",forest[x][y])
            }
            else {
                print!("\x1b[90m{}\x1b[0m",forest[x][y])
            }
        }
        println!() 
    }

    println!("part1: {}",trees.len());
}
