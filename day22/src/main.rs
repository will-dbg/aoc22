use std::io::{BufRead, BufReader};
use std::fs::File;
type  Point = (i32,i32);
fn main() {
    let  lines : Vec<String> = 
    BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap()).collect();

    let map_rows : Vec<&String> = lines.iter().take_while(|l|!l.is_empty()).collect();
    let w : i32 = map_rows.iter().map(|l|l.len()).max().unwrap()as i32;
    let h : i32 = map_rows.len()as i32;
    let mut map = vec![vec![Cell::__;w as usize];h as usize];

    for y  in 0..h{
        let row :Vec<char> = map_rows[y as usize].chars().collect();
        for x in 0..row.len(){
            map[y as usize][x] = match row[x] {
                '.' => Cell::Open,
                '#' => Cell::Wall,
                _ => Cell::__
            }
        }
    }
    println!("w {}, h {}",w,h);
    let instructions = parse_instructions(lines.last().unwrap());
    // println!("{:?}",instructions);
    // for row in &map {
    //     for cell in row {
    //         print!("{:?}",cell);
    //     }
    //     println!()
    // }

    let mut pt:Point = (map[0].iter().enumerate().find(|(_,v)|**v == Cell::Open).unwrap().0 as i32,0);
    let mut hd = Heading::Right;
    for ins in instructions {
        println!("{:?},{:?} {:?} {:?}",pt.0+1,pt.1+1,ins,hd);
        match ins {
            
            Instruction::Turn(t) => {
                hd = hd.turn(t);
            },
            Instruction::Move(mut dist) => {
                let mut last_open = pt;
                while dist > 0 {
                    let  (mut x, mut y)  = next(pt, hd);
                    let xx = (x+w-1).rem_euclid(w-1);
                    let yy = (y+h-1).rem_euclid(h-1);
                    let nxt_cell = map[yy as usize][xx as usize];
                    match nxt_cell {
                        Cell::Wall => {pt=last_open;println!("hit a wall");break;},
                        Cell::Open => {last_open = (xx,yy);pt = (xx,yy); dist-=1;println!("{:?}",pt);},
                        Cell::__ => {pt = (xx,yy); println!("skip {:?}",pt);}
                    }
                }
            }
        }
    }

    pt.0 +=1;
    pt.1 +=1;


    println!("{:?} {:?}",pt,hd);
    println!("part1: {:?}",1000* pt.1 + 4 * pt.0 + hd.value());
}

fn next(pt : Point, hd : Heading) -> Point{
    match hd {
        Heading::Up => (pt.0,pt.1-1),
        Heading::Right => (pt.0+1,pt.1),
        Heading::Down => (pt.0,pt.1+1),
        Heading::Left => (pt.0-1,pt.1)
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction>{
    let chars :Vec<char>= s.chars().collect();
    let mut instructions = Vec::new();
    let mut first_digit = Some(0);
    for x in 0..chars.len() {
        let ch = chars[x];
        match (ch,first_digit) {
            (c,y) if !c.is_digit(10) => {
                if y.is_some() {
                    let z = chars[y.unwrap()..x].iter().collect::<String>()
                    .parse()
                    .unwrap();
                    first_digit = None;
                    instructions.push(Instruction::Move(z));
                }
                
                instructions.push(Instruction::Turn(match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!()
                }));
            },
            (a, None) if a.is_digit(10) => {
                first_digit = Some(x);
            },
            (a,Some(_)) if a.is_digit(10) => (),
            _ => panic!()
        }
    }
    if first_digit.is_some() {
        let z = chars[first_digit.unwrap()..chars.len()].iter().collect::<String>()
        .parse()
        .unwrap();
        instructions.push(Instruction::Move(z));
    }
    instructions
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell{
    Wall,
    Open,
    __
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "█"),
            Self::Open => write!(f, " "),
            Self::__ => write!(f, "░")
        }
    }
}


#[derive(Debug)]
enum Instruction{
    Turn(Direction),
    Move(usize)
}

trait Value {
    fn value(self) -> i32;
    fn from_value(v: i32) -> Self;
}

#[derive(Debug)]
enum Direction{
    Left,
    Right
}
impl Value for Direction{
    
    fn value(self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }

    fn from_value(v: i32) -> Self {
        match v {
            -1 => Direction::Left,
            1 => Direction::Right,
            _ => panic!()
        }
    }
}

#[derive(Debug,Clone, Copy)]
enum Heading {
    Up,
    Right,
    Down,
    Left
}

impl Value for Heading{
    fn value(self) -> i32 {
        match self {
            Heading::Up => 3,
            Heading::Right => 0,
            Heading::Down => 1,
            Heading::Left => 2,
        }
    }

    fn from_value(v: i32) -> Self {
        match v {
            3 => Heading::Up,
            0 => Heading::Right,
            1 => Heading::Down,
            2 => Heading::Left,
            -1 => Heading::Up,
            _ => panic!()
        }
    }
}

impl Heading{
    fn turn(self, dir :  Direction) -> Heading {
        Heading::from_value((self.value() + dir.value()) % 4)
    }
}