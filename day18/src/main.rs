use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

type Cube = (i32,i32,i32);

fn main() {
    let cubes : Vec<Cube> = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|v|{
        let l = v.unwrap();
        let i = l.split(',').map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        (i[0],i[1],i[2])
    }   
    ).collect();

    let mut counts = HashMap::new();
    for cube in &cubes{
        let mut n = HashSet::new();
        for x in neighbour_cubes(cube){
            n.insert(x);
        }
        counts.insert(cube, n);
    }

    for cube in &cubes {
        for neighbour in neighbour_cubes(cube){
            if let Some(c) = counts.get_mut(&neighbour){
                c.remove(&cube);
            }
        }
    }

    let p1 = counts.values().fold(0, |a,y|a+ y.len());
    println!("part1: {}",p1);
}

const NEXT : [Cube; 6] = [
    (1,0,0),
    (-1,0,0),
    (0,1,0),
    (0,-1,0),
    (0,0,1),
    (0,0,-1)
    ];


fn neighbour_cubes(c1: &Cube) -> [Cube;6] {
    NEXT.map(|n|
        ( 
            c1.0 + n.0, 
            c1.1 + n.1, 
            c1.2 + n.2
        )
    )
}
