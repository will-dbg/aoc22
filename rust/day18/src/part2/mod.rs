use std::collections::{HashSet,VecDeque};

use crate::Cube;

type Point = (usize,usize,usize);

pub fn part_2(cubes : &Vec<Cube>, bbox : Point) -> HashSet<Cube> {
    let mut grid = vec![vec![vec![false;bbox.2];bbox.1];bbox.0];
    cubes.iter().for_each(|cube|
        {
            let pt = cube.to_point();
            grid[pt.0][pt.1][pt.2] = true});

    let mut seen = HashSet::new();
    let mut edge_cubes : HashSet<Cube> = HashSet::new();
    let mut q : VecDeque<Point>  = VecDeque::new();
    q.push_front((0,0,0));
    
    while let Some(curr) = q.pop_front(){
        curr.neighbours(bbox)
            .iter()
            .filter(|n| seen.insert(**n))
            .for_each(|p|{
                if grid[p.0][p.1][p.2]{
                    edge_cubes.insert((p.0 as i32,p.1 as i32,p.2 as i32));
                }
                else {
                    q.push_front(*p);
                }
            })
    }
    
    edge_cubes

}

trait Neighbourhood 
where Self : Sized {
    fn neighbours(self, bbox : Point) -> Vec<Self>;
}

const NEXT : [Point; 6] = [
    (1,0,0),
    (usize::MAX,0,0),
    (0,1,0),
    (0,usize::MAX,0),
    (0,0,1),
    (0,0,usize::MAX)
    ];

impl Neighbourhood for Point {
    fn neighbours(self, bbox : Point) -> Vec<Point> {
        NEXT.iter()
            .filter_map(|n| {
                let p = (self.0.wrapping_add(n.0),self.1.wrapping_add(n.1),self.2.wrapping_add(n.2));
                match p {
                    (x,y,z) if 
                            x < bbox.0 && x.abs_diff(self.0) <= 1 &&
                            y < bbox.1 && y.abs_diff(self.1) <= 1 &&
                            z < bbox.2 && z.abs_diff(self.2) <= 1 
                            => Some((x,y,z)),
                    _ => None
                }
            }
            ).collect()
    }
}

trait Pointable {
    fn to_point(self) -> Point;
}

impl Pointable  for Cube {
    fn to_point(self) -> Point {
        (self.0 as usize, self.1 as usize, self.2 as usize)
    }
}