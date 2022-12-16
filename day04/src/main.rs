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

fn challenge1(input: Vec<String>) -> i32 {
    let mut count = 0;
    for line in input {
        let ranges: Vec<i32> = line.split(&['-', ',']).map(|c| c.parse::<i32>().unwrap()).collect();
        if ranges.len() != 4 {
            panic!("bad ranges list for {}", line);
        }
        if ranges[0] >= ranges[2] && ranges[1] <= ranges[3] {
            println!(
                "{}-{} is inside {}-{}",
                ranges[0], ranges[1], ranges[2], ranges[3]
            );
            count += 1;
        } else if ranges[0] <= ranges[2] && ranges[1] >= ranges[3] {
            println!(
                "{}-{} is inside {}-{}",
                ranges[2], ranges[3], ranges[0], ranges[1]
            );
            count += 1;
        }
    }
    count
}

fn challenge2(input: Vec<String>) -> i32 {
    let mut count = 0;
    for line in input {
        let ranges: Vec<i32> = line.split(&['-', ',']).map(|c| c.parse::<i32>().unwrap()).collect();
        if ranges.len() != 4 {
            panic!("bad ranges list for {}", line);
        }
        if ranges[0] >= ranges[2] && ranges[1] <= ranges[3] {
            println!(
                "{}-{} is inside {}-{}",
                ranges[0], ranges[1], ranges[2], ranges[3]
            );
            count += 1;
            continue;
        }
        if ranges[0] <= ranges[2] && ranges[1] >= ranges[3] {
            println!(
                "{}-{} is inside {}-{}",
                ranges[2], ranges[3], ranges[0], ranges[1]
            );
            count += 1;
            continue;
        }
        if ranges[0] <= ranges[2] && ranges[1] >= ranges[2] {
            count += 1;
            println!("ranges {}-{} and {}-{} overlap (1)", ranges[0], ranges[1], ranges[2], ranges[3]);
            continue;
        }
        if ranges[0] <= ranges[2] && ranges[2] <= ranges[1] {
            println!("ranges {}-{} and {}-{} overlap (2)", ranges[0], ranges[1], ranges[2], ranges[3]);
            count += 1;
            continue;
        }
        if ranges[0] <= ranges[2] && ranges[3] <= ranges[1] {
            println!("ranges {}-{} and {}-{} overlap (3)", ranges[0], ranges[1], ranges[2], ranges[3]);
            count += 1;
            continue;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            challenge1(vec![
                "2-4,6-8".to_string(),
                "2-3,4-5".into(),
                "5-7,7-9".into(),
                "2-8,3-7".into(),
                "6-6,4-6".into(),
                "2-6,4-8".into()
            ]),
            2
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            challenge2(vec![
                "2-4,6-8".to_string(),
                "2-3,4-5".into(),
                "5-7,7-9".into(),
                "2-8,3-7".into(),
                "6-6,4-6".into(),
                "2-6,4-8".into()
            ]),
            4
        )
    }


    //    #[test]
    //    fn test_2() {
    //        assert_eq!(
    //            challenge1(vec!["A Y".to_string(), "B X".into(), "C Z".into()]),
    //            15
    //        )
    //    }
    //
    //    #[test]
    //    fn test_3() {
    //        assert_eq!(
    //            challenge2(vec!["A Y".to_string(), "B X".into(), "C Z".into()]),
    //            12
    //        )
    //    }
}
