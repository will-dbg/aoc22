use std::io::{BufRead, BufReader};
mod part2;
use std::fs::File;
use std::collections::{HashMap, HashSet};
type Cube = (i32,i32,i32);

fn main() {
    

    let lines = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines();
    let mut bbox =  (0usize,0usize,0usize);
    let cubes : Vec<Cube> = lines.map(|v|{
        let l = v.unwrap();
        let i = l.split(',').map(|x|x.parse().unwrap()).collect::<Vec<i32>>();
        bbox.0 = std::cmp::max(bbox.0 ,(i[0]+1) as usize);
        bbox.1 = std::cmp::max(bbox.1 ,(i[1]+1) as usize);
        bbox.2  = std::cmp::max(bbox.2,(i[2]+1) as usize);

        (i[0],i[1],i[2])
    }   
    ).collect();

    println!("{:?}",bbox);
    let face_counts  = count_faces(&cubes);
    let p1 : usize = face_counts.values().sum();
    println!("part1: {}",p1);

    let p2Filter  = part2::part_2(&cubes, bbox);

    let p2 : usize = face_counts
        .iter()
        .filter(|(k,_)|p2Filter.contains(*k))
        .map(|(_,v)|v)
        .sum();

        println!("part2: {}",p2);

}


fn count_faces(cubes : &Vec<Cube>) -> HashMap<Cube,usize> {
    let mut counts = HashMap::new();
    for cube in cubes{
        let mut n = HashSet::new();
        for x in neighbour_cubes(cube){
            n.insert(x);
        }
        counts.insert(cube, n);
    }

    for cube in cubes {
        for neighbour in neighbour_cubes(cube){
            if let Some(c) = counts.get_mut(&neighbour){
                c.remove(&cube);
            }
        }
    }
    let mut counts2 = HashMap::new();

    counts.iter().for_each(|(k,v)|{
        counts2.insert(**k, v.len());
    });
    counts2
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
