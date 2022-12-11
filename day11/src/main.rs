use std::default;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;
//15390 low
fn main() {
    let lines : Vec<String> = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap()).collect(); 

    println!("part1: {}",solve(&lines,3,20));
    println!("part2: {}",solve(&lines,1,10_000));
}

fn solve(lines: &Vec<String>, div: usize, n_rounds : usize) -> usize{
    let mut monkeys : Vec<Monkey> = lines.chunks(7).map(|m|parse_monkey(m)).collect();
    let mut counts = vec![0;monkeys.len()];
    let m = monkeys.iter().map(|monkey| monkey.test).product();
    println!("{}",m);
    (0..n_rounds).for_each(|_|round(&mut monkeys, &mut counts, div, m));
    counts.sort_unstable();
    counts.iter().rev().take(2).product()
}

fn round(monkeys: &mut Vec<Monkey>, counts : &mut Vec<usize>, div : usize, worry_mod : usize ){
    for i in 0..monkeys.len(){
        counts[i] += monkeys[i].q.len();
        while let Some(item) = & monkeys[i].q.pop_front(){
            let next_worry= (monkeys[i].op.apply(*item) / div) % worry_mod;
            let target = 
            if next_worry % monkeys[i].test == 0 {
                monkeys[i].if_true
            }
            else {
                monkeys[i].if_false
            };
            monkeys[target].q.push_back(next_worry)
        }
    }
    
}

fn parse_monkey(monkey_dec : &[String]) -> Monkey{
    let nums : Vec<usize> = monkey_dec[1]
    .split_once(": ")
    .unwrap().1.split(", ").map(|x|x.parse::<usize>().unwrap()).collect();
    let op_line = monkey_dec[2].as_str().split_once("new = old ").unwrap().1;
    let op = match op_line {
        "* old" => Operation::Square,
        _ if op_line.contains("*") => Operation::Mul(op_line.split_once("* ").unwrap().1.parse().unwrap()),
        _ if op_line.contains("+") => Operation::Add(op_line.split_once("+ ").unwrap().1.parse().unwrap()),
        _ => panic!()
    };
    let test = monkey_dec[3].split_at(21).1.parse().unwrap();
    let if_true = monkey_dec[4].split_at(29).1.parse().unwrap();
    let if_false = monkey_dec[5].split_at(30).1.parse().unwrap();
    Monkey::new(&nums, if_true, if_false, test, op)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation{
    Add(usize),
    Mul(usize),
    Square
}

impl Operation{
    fn apply(&self, item : usize ) -> usize{
        match self {
            Operation::Add(v) => item + v,
            Operation::Mul(v) => item * v,
            Operation::Square => item * item
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey{
    q : VecDeque<usize>,
    if_true : usize,
    if_false : usize,
    test : usize,
    op : Operation
}

impl Monkey {
    fn new(start_items : &[usize], if_true : usize, if_false : usize, test: usize, op : Operation ) -> Self
    {        
        let mut q : VecDeque<usize> = VecDeque::new();
        for x in start_items {
            q.push_back(x.clone());
        }
        Self{
            q,
            if_false,
            if_true,
            test,
            op
        }
    }
}