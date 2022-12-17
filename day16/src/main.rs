use std::collections::{HashSet, HashMap};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut graph = HashMap::new();

    BufReader::new(File::open("test.txt").expect("open failed"))
    .lines()
    .map(|l|parse_valve(l.unwrap().as_str()))
    .for_each(move |valve| {graph.insert(valve.key.clone(), valve);} );

    let mut curr = "AA";
}

fn parse_valve(txt:&str) -> Valve{
    let mut words = txt.split(" ").into_iter();
    let mut key = None;
    let mut flow_rate = None;
    let mut outflow = HashSet::new();

    for i in 0.. {
        if let Some(word) = words.next(){
            if i == 1 {
                key = Some(word.to_owned());
            }
            if i == 4 {
                flow_rate = Some(word.split_terminator(&['=',';']).collect::<Vec<&str>>().iter().nth(1).unwrap().parse().unwrap())
            }
            if i >= 9 {
                outflow.insert(word.chars().take_while(|c|*c!=',').collect());
            }
        } else { break; }
    }
    Valve { key: key.unwrap(), flow_rate : flow_rate.unwrap(), outflow }
}



#[derive(Debug, Clone)]
struct Valve{
    key : String,
    flow_rate : usize,
    outflow : HashSet<String>
}