use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let lines : Vec<String> = BufReader::new(File::open("input.txt").expect("open failed"))
        .lines()
        .map(|l|l.unwrap()).collect();

    let lineRefs = lines.iter().map(|l|l.as_str());

    let mut dirs = vec![Directory::new(None)];
    let mut cwd = 0;   

    for line in lineRefs {
        match line {
            "$ cd /" => {
                cwd = 0;
            },
            "$ cd .." => {
                cwd = match  dirs[cwd].parent{
                     Some(x) => x,
                     _ => 0
                };
            },
            _ if line.starts_with("$ cd ") => {
                let dir_name = line.strip_prefix("$ cd ").unwrap();
                cwd = dirs[cwd].children[dir_name];
            }
            "$ ls" => (),
            _ if line.starts_with("dir") => {
                let dir_id = dirs.len();
                dirs.push(Directory::new(Some(cwd)));
                dirs[cwd].children.insert(line.strip_prefix("dir ").unwrap(), dir_id);
            },
            _ => {
                let (num, _) = line.split_once(" ").unwrap();
                dirs[cwd].size += num.parse::<usize>().unwrap();
            }
        }
    }

    let part1 = dirs.iter().fold(0, |sz,dir|
        {let szz = dir.get_size(&dirs);
        if szz < 100000 {
            return sz+ szz;
        }
        else {return sz}
    }
    );

    println!("part1: {}",part1);
}

#[derive(Debug)]
struct Directory<'a> {
    parent: Option<usize>,
    size: usize,
    children: HashMap<&'a str, usize>
}

impl Directory<'_> {
    fn new(parent: Option<usize>) -> Self{
        Self {
            children: HashMap::new(),
            size:0,
            parent: parent 
        }
    }

    fn get_size<'b>(&self, vecs : &Vec<Directory<'b>>) -> usize {
        self.size + 
            self.children.values().map(
                |c|
                {
                    vecs[*c].get_size(vecs)
                }
                ).sum::<usize>()
    }
}