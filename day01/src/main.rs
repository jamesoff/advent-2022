mod data;

fn main() {
    println!("challenge 1: {}", challenge1(data::input()));
    println!("challenge 2: {}", challenge2(data::input()));
}

fn challenge1(input: Vec<&str>) -> i32 {
    let mut current_total = 0;
    let mut max_seen = 0;
    for item in input {
        if item.len() == 0 {
            if current_total > max_seen {
                max_seen = current_total;
            }
            current_total = 0;
            continue;
        }
        current_total += item.parse::<i32>().expect("Failed to parse int");
    }
    if current_total > max_seen {
        max_seen = current_total;
    }
    max_seen
}


fn challenge2(input: Vec<&str>) -> i32 {
    let mut totals: Vec<i32> = vec![];
    let mut current_total = 0;
    for item in input {
        if item.len() == 0 {
            totals.push(current_total);
            current_total = 0;
            continue;
        }
        current_total += item.parse::<i32>().expect("Failed to parse int");
    }
    totals.push(current_total);
    totals.sort();
    totals.reverse();
    totals[0] + totals[1] + totals[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(
            challenge1(vec![
                "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000",
                "", "10000",
            ]),
            24000
        )
    }

    #[test]
    fn test_data2() {
        assert_eq!(
            challenge2(vec![
                "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000",
                "", "10000",
            ]),
            45000
        )
    }
}
