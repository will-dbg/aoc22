use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Move{
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Game{
    Win,
    Lose,
    Draw
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").expect("open failed")).lines().map(|l|l.unwrap());
    let mut score1 =0;
    let mut score2 =0;
    for line in lines {
        let opponent = their_move(line.chars().nth(0));
        let me = line.chars().nth(2);
        
        score1 = score1 + score_1(&opponent,&my_move(me));
        score2 = score2 + score_2(parse_outcome(me),opponent);
    }
    println!("part1: {}",score1);
    println!("part2: {}",score2);
}

fn their_move(c: Option<char> ) -> Move {
    match c {
        Some('A') => Move::Rock,
        Some('B') => Move::Paper,
        Some('C') => Move::Scissors,
        _ => panic!("nope")
    }
}

fn my_move(c: Option<char> ) -> Move {
    match c {
        Some('X') => Move::Rock,
        Some('Y') => Move::Paper,
        Some('Z') => Move::Scissors,
        _ => panic!("nope")
    }
}

fn score_1(them: &Move, me: &Move ) -> i32{
    return match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    } +
    match (me,them) {
        (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper) => 6,
        (x, y) if x == y => 3,
        _ => 0
    };
}


fn parse_outcome(c: Option<char> ) -> Game {
    match c {
        Some('X') => Game::Lose,
        Some('Y') => Game::Draw,
        Some('Z') => Game::Win,
        _ => panic!("nope")
    }
}


fn score_2(outcome: Game, them: Move ) -> i32{
    let me = pick_my_move(outcome,&them);
    let score = score_1(&them, me);
    return score
}


fn pick_my_move(outcome: Game, them: &Move ) -> &Move {
    return match outcome {
        Game::Lose => match them {
            Move::Rock => &Move::Scissors,
            Move::Paper => &Move::Rock,
            Move::Scissors => &Move::Paper
        },
        Game::Win => match them {
            Move::Scissors => &Move::Rock,
            Move::Rock => &Move::Paper,
            Move::Paper => &Move::Scissors
        },
        _ => them
    }
}