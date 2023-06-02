use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

fn main() {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 11];
    let mut stacks2: Vec<VecDeque<char>> = vec![VecDeque::new(); 11];
    BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .for_each(|line| match line.unwrap().as_ref() {
        l if l[0] == '[' => {
            parse_stack_row(l, &mut stacks);
            parse_stack_row(l, &mut stacks2);
        },
        l if l[..4] == *"move" => {
            let nums: Vec<usize> = l.split(' ').flat_map(|f| f.parse::<usize>()).collect();
            process_p1(l,&mut stacks,&nums);
            process_p2(l, &mut stacks2, &nums)
        },
        _ => () 
    });
    print!("P1 ");
    for x in (0..9){
        if let Some(x) = stacks[x].front() {
            print!("{}",x)
        }
    }
    print!("\nP2 ");
    for x in (0..9){
        if let Some(x) = stacks2[x].front() {
            print!("{}",x)
        }
    }
}

fn parse_stack_row(str: &str, mut stack :  &mut Vec<VecDeque<char>>){
    let a : Vec<char> = str.chars().collect();
            for i in 0..stack.len() {
                if let Some(char) = a.get(1+i * 4){
                    if *char != ' ' {
                        stack[i].push_back(*char);
                    }
                }
            }
}

fn process_p1(l : &str, mut stacks : &mut  Vec<VecDeque<char>>, nums : &Vec<usize>){
    
    for _ in (0..nums[0]){
        if let Some(val) = stacks[nums[1]-1].pop_front(){
            stacks[nums[2] - 1].push_front(val)
        }
    }
}

fn process_p2(l : &str, mut stacks2 : &mut Vec<VecDeque<char>>, nums : &Vec<usize>){
    let mut tmp : Vec<char> = (0..nums[0]).map(|_| stacks2[nums[1] - 1].pop_front().unwrap()).collect();
            
    for _ in (0..nums[0]){
        stacks2[nums[2] - 1].push_front(tmp.pop().unwrap());
    }
}
