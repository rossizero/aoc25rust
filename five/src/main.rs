use std::cmp::{max, min};
use utils;

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let mut ids: Vec<i64> = Vec::new();

        for mut line in lines {
            if let Ok(ref mut l) = line {
                let pos = l.find("-");
                if pos.is_some() {
                    let two = l.split_off(pos.unwrap() + 1);
                    let one = l[..l.len() - 1].parse::<i64>();
                    let two = two.parse::<i64>();

                    if let (Ok(a_num), Ok(b_num)) = (one, two) {
                        ranges.push((a_num, b_num));
                    }
                } else {
                    let num = l.parse::<i64>();
                    if let Ok(num) = num {
                        ids.push(num);
                    }
                }
            }
        }

        let res1 = one(&ranges, &ids);
        let res2 = two(ranges);

        println!("result 1: {} 2: {}", res1, res2);
    }
}

fn one(ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> i32 {
    let mut result = 0;

    for id in ids {
        for range in ranges {
            if *id >= range.0 && *id <= range.1 {
                result += 1;
                break;
            }
        }
    }
    return result;
}

fn two(mut ranges: Vec<(i64, i64)>) -> i64 {
    ranges.sort_by_key(|r| r.0);

    let mut result = 0;
    let mut resulting_ranges: Vec<(i64, i64)> = Vec::new();

    let mut current = ranges[0];

    for &range in &ranges[1..] {
        if range.0 <= current.1 && range.1 >= current.0 {
            current = (min(current.0, range.0), max(current.1, range.1));
        } else {
            resulting_ranges.push(current);
            current = range;
        }
    }

    resulting_ranges.push(current);

    for range in resulting_ranges {
        result += range.1 - range.0 + 1;
    }
    return result;
}
