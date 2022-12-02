use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSE_POINTS: i32 = 0;

const ROCK_A: &str = "A";
const PAPER_A: &str = "B";
const SCISSORS_A: &str = "C";

const ROCK_B: &str = "X";
const PAPER_B: &str = "Y";
const SCISSORS_B: &str = "Z";

const MUST_WIN: &str = "Z";
const MUST_DRAW: &str = "Y";
const MUST_LOSE: &str = "X";

fn main() {
    println!("challenge 1 is {}", challenge1(load_input()));
    println!("challenge 2 is {}", challenge2(load_input()));
}

fn load_input() -> Vec<String> {
    let file_handle = File::open("input.txt").expect("failed to open 'input.txt'");
    let reader = io::BufReader::new(file_handle);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn challenge1(input: Vec<String>) -> i32 {
    let mut total = 0;
    for entry in input {
        let play: Vec<&str> = entry.split(" ").collect();
        let points = match play[0] {
            ROCK_A => match play[1] {
                PAPER_B => PAPER_POINTS + WIN_POINTS,
                ROCK_B => ROCK_POINTS + DRAW_POINTS,
                SCISSORS_B => SCISSORS_POINTS + LOSE_POINTS,
                _ => panic!("bad play"),
            },
            PAPER_A => match play[1] {
                PAPER_B => PAPER_POINTS + DRAW_POINTS,
                ROCK_B => ROCK_POINTS + LOSE_POINTS,
                SCISSORS_B => SCISSORS_POINTS + WIN_POINTS,
                _ => panic!("bad play"),
            },
            SCISSORS_A => match play[1] {
                PAPER_B => PAPER_POINTS + LOSE_POINTS,
                ROCK_B => ROCK_POINTS + WIN_POINTS,
                SCISSORS_B => SCISSORS_POINTS + DRAW_POINTS,
                _ => panic!("bad play"),
            },
            _ => panic!("bad play"),
        };
        total += points;
    }
    total
}

fn challenge2(input: Vec<String>) -> i32 {
    let mut total = 0;
    for entry in input {
        let play: Vec<&str> = entry.split(" ").collect();
        let points = match play[0] {
            ROCK_A => match play[1] {
                MUST_WIN => PAPER_POINTS + WIN_POINTS,
                MUST_DRAW => ROCK_POINTS + DRAW_POINTS,
                MUST_LOSE => SCISSORS_POINTS + LOSE_POINTS,
                _ => panic!("bad play"),
            },
            PAPER_A => match play[1] {
                MUST_DRAW => PAPER_POINTS + DRAW_POINTS,
                MUST_LOSE => ROCK_POINTS + LOSE_POINTS,
                MUST_WIN => SCISSORS_POINTS + WIN_POINTS,
                _ => panic!("bad play"),
            },
            SCISSORS_A => match play[1] {
                MUST_LOSE => PAPER_POINTS + LOSE_POINTS,
                MUST_WIN => ROCK_POINTS + WIN_POINTS,
                MUST_DRAW => SCISSORS_POINTS + DRAW_POINTS,
                _ => panic!("bad play"),
            },
            _ => panic!("bad play"),
        };
        total += points;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(challenge1(vec!["A Y".to_string()]), 8)
    }

    #[test]
    fn test_2() {
        assert_eq!(
            challenge1(vec!["A Y".to_string(), "B X".into(), "C Z".into()]),
            15
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            challenge2(vec!["A Y".to_string(), "B X".into(), "C Z".into()]),
            12
        )
    }
}
