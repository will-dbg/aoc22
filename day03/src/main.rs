use std::io::{BufRead, BufReader};
use std::fs::File;
use bit_vec::BitVec;


fn main() {   

    let lines = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap())
    .collect::<Vec<String>>();

    let part1 = lines.iter().fold(0,|acc, line| acc+ score_line(&line));

    


    let mut part2 = 0;
    for chunk in lines.chunks(3) {
        let mut a = group_vec(&chunk[0]);
        a.and(&group_vec(&chunk[1]));
        a.and(&group_vec(&chunk[2]));
        for x in 0..53 {
            if a[x] {
                part2 =  part2 + x;
                break;
            }
        }        
    }

    println!("part1: {}",part1);
    println!("part2: {}",part2);
}

fn priority(c: char) -> usize {
    let n = c as usize;
    if n <= 90{
        return n - 38
    }
    return n - 96
}


fn score_line(line: &str) -> usize{
    let mut bv = BitVec::from_elem(53, false);
    let chars : Vec<char> = line.chars().collect();
    let c = chars.len();
    for i in 0..c {
        let ch = chars[i];
        let p = priority(ch);
        if i < c /2 {
            bv.set(p,true);
        }
        else{
            if bv[p] {
                return p
            }
        }
    }
    panic!()
}

fn group_vec(line: &str) -> BitVec {
    let mut bv = BitVec::from_elem(53, false);
    let chars : Vec<char> = line.chars().collect();
    let c = chars.len();
    for i in 0..c {
        let ch = chars[i];
        let p = priority(ch);
        bv.set(p,true);
    }
    return bv;
}