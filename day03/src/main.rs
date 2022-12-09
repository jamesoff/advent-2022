use std::collections::hash_set::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn main() {
    println!("challenge 1 is {}", challenge1(load_input()));
    println!("challenge 2 is {}", challenge2(load_input()));
}

fn load_input() -> Vec<String> {
    let file_handle = File::open("input.txt").expect("failed to open 'input.txt'");
    let reader = io::BufReader::new(file_handle);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn score_char(c: &char) -> i32 {
    let value: i32 = c
        .to_digit(36)
        .expect("failed to get digit value")
        .try_into()
        .expect("invalid for i32");
    match c.is_ascii_uppercase() {
        false => value - 9,
        true => value + 17,
    }
}

fn challenge1(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in input {
        let total_items = line.len();
        if total_items % 2 != 0 {
            panic!("entry {} does not have even length", line);
        };
        let first = &line[0..total_items / 2];
        let second = &line[total_items / 2..];

        let mut first_items: Vec<char> = first.chars().collect();
        first_items.sort();
        first_items.dedup();
        let first_hash: HashSet<char> = first_items.into_iter().collect();

        let mut second_items: Vec<char> = second.chars().collect();
        second_items.sort();
        second_items.dedup();
        let second_hash: HashSet<char> = second_items.into_iter().collect();

        let in_both = first_hash.intersection(&second_hash);
        sum = sum + in_both.into_iter().map(|c| score_char(c)).sum::<i32>();
    }
    sum
}

fn line_to_items(line: &String) -> HashSet<char> {
    let mut items: Vec<char> = line.chars().collect();
    items.sort();
    items.dedup();
    items.into_iter().collect()
}

fn challenge2(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for group in input.chunks(3) {
        let first_items = line_to_items(&group[0]);
        let second_items = line_to_items(&group[1]);
        let third_items = line_to_items(&group[2]);
        let in_first_second: HashSet<char> =
            first_items.intersection(&second_items).copied().collect();
        let in_all: Vec<char> = third_items.intersection(&in_first_second).copied().collect();
        if in_all.len() != 1 {
            panic!("bad length for in_all: {}", in_all.len());
        }
        sum += score_char(&in_all[0]);
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(challenge1(vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string()]), 16)
    }

    #[test]
    fn test_2() {
        assert_eq!(
            challenge1(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ]),
            157
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            challenge2(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ]),
            70
        )
    }

    #[test]
    fn test_char() {
        assert_eq!(score_char(&'p'), 16);
        assert_eq!(score_char(&'P'), 42);
        assert_eq!(score_char(&'L'), 38);
    }
}
