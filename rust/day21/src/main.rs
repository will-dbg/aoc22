use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
fn main() {
    let mut monkeez = HashMap::new();
    BufReader::new(File::open("test.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap())
    .for_each(|l|{
        let (name,rest) = l.as_str().split_once(": ").unwrap();
        let words : Vec<&str> = rest.split(" ").collect();
        if words.len() == 3 {
            let lName = words[0];
            let rName = words[2];
            let op =  match words[1] {
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "/" => Operation::Div,
                "*" => Operation::Mul,
                _ => panic!()
            };
            monkeez.insert(name.to_owned(), Evaluation::Calculation(lName.to_owned(), rName.to_owned(), op));
            
        }
        else {
            monkeez.insert(name.to_owned(),Evaluation::Value(words[0].parse().unwrap()));
        }
    });

    let root =monkeez.get("root").unwrap();

    let p1 = root.evaluate(&monkeez);
    println!("p1: {}",p1);
}

enum Evaluation{
    Value(i64),
    Calculation(String, String, Operation),
}

enum Operation{
    Add,
    Sub,
    Mul,
    Div
}

struct Monkey{
    eval : Evaluation
}

impl Evaluation {
    fn evaluate(&self, monkees: &HashMap<String,Evaluation>) -> i64{
        match &self {
            Evaluation::Value(y) => *y,
            Evaluation::Calculation(left, right, op) => {
                let l = monkees.get(left.as_str()).unwrap().evaluate(monkees);
                let r = monkees.get(right.as_str()).unwrap().evaluate(monkees);
                if left.as_str() == "humn" || right.as_str() == "humn"{
                    println!("hmn")
                }
                return match op {
                    Operation::Add => l+r,
                    Operation::Sub => l-r,
                    Operation::Mul => l*r,
                    Operation::Div => l/r,
                }
            }
        }
    }

    
}