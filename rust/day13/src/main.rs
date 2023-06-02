use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;
use std::vec;

fn main() {
    let mut packets = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap())
    .filter(|l|l != "")
    .map(|line|{let l = line.as_str(); return parse_row(&l[0..l.len()]); })
    .collect::<Vec<ListElem>>();


    let p1 = packets
    .chunks(2)
    .enumerate()
    .fold(0 as usize, |ct ,(s,chunk)|{
        return match chunk[0].cmp(&chunk[1]) {
            Ordering::Less => ct+s+1,
            _ => ct
        }
    });

    println!("part1: {}",p1);

    let n_two = packets.iter().filter(|p| p < &&ListElem::List(vec![ListElem::Num(2)])).count()+1;
    let n_six = packets.iter().filter(|p| p < &&ListElem::List(vec![ListElem::Num(6)])).count()+2;

    println!("part2: {:?}",n_two * n_six);

    
}


fn parse_row<'a>(row : &str) -> ListElem {
    let mut lineage : Vec<Vec<ListElem>> = vec![vec![]];
    let mut p = 0;
    let mut str : Option<Vec<char>> = None;
    
    for ch in row.to_owned().chars(){
        match ch {
            '[' => {
                p +=1;
                lineage.push(Vec::new());
                str = None;
                
            },
            ']' => {
                if str.is_some() {
                    lineage[p].push(ListElem::Num(str.unwrap().into_iter().collect::<String>().parse().unwrap()));
                    str = None
                }
                let last = lineage[p].to_vec();
                if(p >= 1){
                    lineage[p-1].push(ListElem::List( last));
                    lineage.pop();
                    p -=1;
                }
                
            },
            ',' => {
                if str.is_some() {
                    lineage[p].push(ListElem::Num(str.unwrap().into_iter().collect::<String>().parse().unwrap()));
                    str = None
                }
            }
            _ => {
                match str {
                    Some(x) => {
                        let mut y = x.to_owned();
                        y.push(ch);
                        str = Some(y)

                    },
                    _ => str = Some(vec![ch])
                }
            }
        }
    }
    ListElem::List(lineage[0].to_owned())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElem {
    Num(usize),
    List(Vec<ListElem>)
}

impl Ord for ListElem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListElem::Num(a), ListElem::Num(b)) => a.cmp(b),
            (ListElem::List(a), ListElem::List(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (ListElem::Num(_), ListElem::List(b)) if b.len() == 1 => self.cmp(&b[0]),
            (ListElem::Num(a), ListElem::List(_)) => ListElem::List(vec![ListElem::Num(*a)]).cmp(other),
            (ListElem::List(_), ListElem::Num(_)) => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for ListElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}