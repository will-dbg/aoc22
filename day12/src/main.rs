use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
type  Point =  (usize,usize);

fn main() {
    
    let (heights,start,fin) = parse();
    
    println!("part1 {:?}",calc_distance(&heights,start,fin));

    // this is far from optimal... but it works
    let pt2 = heights.iter().filter(|(pt,lvl)| **lvl == 1)
    .filter_map(|(pt,_)|calc_distance(&heights,*pt,fin)).min();

    println!("part2 {:?}",pt2);
}

fn calc_distance(heights: &HashMap<Point,usize>,start: Point, finish : Point ) -> Option<usize>{
    let mut search_edge: HashSet<Point> = vec![start].iter().cloned().collect();
    let mut visited: HashSet<Point> = vec![start].iter().cloned().collect();
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

/*
fn djikstra(game : Game){

    let dist : BinaryHeap<PointDist> = BinaryHeap::new();
    for pt in game.grid.iter().filter(|pt|game.s != pt) {
        if()
    }
}

fn parse_game(lines : & Vec<String> ) -> Game {
    let mut grid : Vec<Point> = Vec::new();
    let mut s = Point{x:0,y:0,h:0};
    let mut e = Point{x:0,y:0,h:0};
    for y in  0..lines.len() {
        grid[y] = lines[y].chars().enumerate().map(|(x, ch)|{
            let pt = Point {x,y,h: ch as usize - 'a' as usize };
            match ch {
                'S' => {s = pt;},
                'E' => {e = pt;},
                _ => ()
            }
            pt
        }
        ).flatten().collect();
    }
    let w = grid.len();
    let h = grid[0].len();
    Game { grid, s, e, w, h }
}

#[derive(Debug,Clone)]
struct Game{
    grid : Vec<Point>,
    s : Point,
    e : Point,
    w : usize,
    h : usize
}


const MAP_NEXT : [(usize, usize); 4] = [(0,1),(1,0),(0,usize::MAX),(usize::MAX,0)];

impl Game {
    fn next_point(&self, path : Option<&Path>) -> Path{
        let mut p = match path  {
            Some(pth) => pth.clone(),
            None => Path::new_(self.s)
        };
        let pt = p.points.front().unwrap();

        let next_bois = MAP_NEXT.iter()
            .map(|(xx,yy)|{
                (pt.x.wrapping_add(1), pt.y.wrapping_add(1))
            })
            .filter(|(x,y)|x < &self.w && y < &self.h && self.height_at(*x,*y).is_some())
            .collect::<Vec<_>>();
        p
    }

    fn height_at(&self,x : usize, y: usize) -> Option<usize> {
        if x >= 0 && x < self.w
            && y >= 0 && y < self.h{
                Some(self.grid[x][y].h)
            }
            else{
                None
            }
    }
}

#[derive(Debug,Clone)]
struct Path{
    points : LinkedList<Point>
}

impl Path {
    fn new() -> Self{
        Self {
            points : LinkedList::new()
        }
    }

    fn new_(point :Point) -> Self{
        let mut p = Path::new();
        p.points.push_front(point);
        return p;
    }
}

#[derive(Debug,Clone, Copy,Eq, PartialEq)]
struct Point{
    x : usize,
    y : usize,
    h : usize
}

#[derive(Eq)]
struct PointDist{
    pt : Point,
    dist : usize
}

impl PartialOrd for PointDist{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointDist {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.dist == other.dist
    }
}

impl Ord for PointDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist > other.dist {
            Ordering::Greater
        }
        else if self.dist == other.dist {
            Ordering::Equal
        }
        else {
            Ordering::Less
        }
    }
}
 */