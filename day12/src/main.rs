use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
type  Point =  (usize,usize);

fn main() {
    
    let (heights,start,fin) = parse();
    
    println!("part1 {:?}",calc_distance(&heights,vec![start],fin));

    // this is far from optimal... but it works
    let pt2 = heights
    .iter()
    .filter(|(_,lvl)| **lvl == 1)
    .map(|(pt,_)|*pt).collect();

    println!("part2 {:?}",calc_distance(&heights,pt2,fin));
}

fn calc_distance(heights: &HashMap<Point,usize>,start: Vec<Point>, finish : Point ) -> Option<usize>{
    let mut search_edge: HashSet<Point> = start.iter().cloned().collect();
    let mut visited: HashSet<Point> = start.iter().cloned().collect();
    let mut stepcost = 0;

    while !search_edge.is_empty(){
        stepcost += 1;
        for from in search_edge.drain().collect::<Vec<_>>() {
            for to in unvisited(&heights, &visited, from) {
                if heights[&to] <= heights[&from] + 1{
                    if to == finish {
                        return Some(stepcost)
                    }
                    visited.insert(to);
                    search_edge.insert(to);
                }   
            }
        }
    }
    None
}

const NEXT : [Point; 4] = [(0,1),(1,0),(usize::MAX,0),(0,usize::MAX)];

fn unvisited(heights: &HashMap<Point,usize>,visited : &HashSet<Point>, me : Point) -> Vec<Point> {
    NEXT.iter()
    .map(|(x,y)|(me.0.wrapping_add(*x),me.1.wrapping_add(*y)))
    .filter(|pt|heights.get(pt).is_some() && !visited.contains(pt))
    .collect()
}

fn parse() -> (HashMap<Point,usize>, Point, Point){
    BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap())
    .enumerate()
    .fold((HashMap::new(), Point::default(), Point::default()),
    |(mut heights, mut start, mut fin),(y,line)|
    {
        line
        .chars()
        .enumerate()
        .for_each(|(x,ch)|{
            let val = match ch {
                'S' => {
                    start = (x,y);
                    1
                },
                'E' => {
                    fin = (x,y);
                    26
                },
                _ => ch as usize - 'a' as usize + 1
            };
            heights.insert((x,y), val);
        });
        (heights,start,fin)
    })
}
