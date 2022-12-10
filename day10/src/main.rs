
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let lines : Vec<String> = BufReader::new(File::open("input.txt").expect("open failed"))
    .lines()
    .map(|l|l.unwrap()).collect(); 

    let mut unpacked_ops : Vec<i32>  = Vec::new();
    // lol
    for line in lines {
        match line.as_str() {
            "noop" => {
                unpacked_ops.push(0)
            },
            o => {
                let n : i32 = o.split_once(" ").unwrap().1.parse().unwrap();
                unpacked_ops.push(0);
                unpacked_ops.push(n);
            }
        }
    }

    let a = ScanArg{
        x_register : 1, i: 1
    };

    let p1 = unpacked_ops.iter().scan(a, |acc,op|{
        let x = acc.x_register;
        let i = acc.i;
        acc.x_register += op;
        acc.i += 1;

        let pix = if (x-1..x+2).contains(&((i-1)%40)) {'#'}else{'.'};
        if i % 40 == 0 {
            println!("{}",pix);
        }
        else {
            print!("{}",pix);
        }


        if i == 20 || i > 20 && (i - 20) % 40 == 0 {
            Some(x*i)
        }
        else {
            Some(0)
        }
        
    })
    .fold(0, |a,x| a+x);

    println!("part1: {:?} ",p1);
}

#[derive(Debug, Clone, Copy)]
struct ScanArg {
    x_register : i32,
    i : i32
}

