use std::io;
use utils;

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let per_line: io::Result<Vec<Vec<i32>>> = lines
            .map(|line| {
                let s = line?;
                let digits: Vec<i32> = s
                    .trim()
                    .chars()
                    .map(|c| if c == '@' { 1 } else { 0 })
                    .collect();
                Ok(digits)
            })
            .collect();

        let a = per_line.unwrap().clone();
        let b = a.clone();

        let (res1, _) = one(a);
        let res2 = two(b);

        println!("result 1: {} 2: {}", res1, res2);
    }
}

fn one(input: Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut result = 0;
    let neighbour_size = 1;
    let width = input.len();
    let height = input[0].len();
    let mut working = input.clone();

    for y in 0..height {
        let y_start = y.saturating_sub(neighbour_size);
        let y_end = (y + neighbour_size + 1).min(height);

        for x in 0..width {
            let x_start = x.saturating_sub(neighbour_size);
            let x_end = (x + neighbour_size + 1).min(width);

            let hood: Vec<&[i32]> = input[y_start..y_end]
                .iter()
                .map(|row| &row[x_start..x_end])
                .collect();

            if input[y][x] == 1 {
                let count: i32 = hood.iter().flat_map(|row| row.iter()).sum();

                if count < 5 {
                    result += 1;
                    working[y][x] = 0;
                }
            }
        }
    }

    return (result, working);
}

fn two(input: Vec<Vec<i32>>) -> i32 {
    let (mut num, mut tmp) = one(input);
    let mut result = num;
    //print_map(&tmp);
    //println!("{num}");

    while num > 0 {
        (num, tmp) = one(tmp);
        result += num;
        //print_map(&tmp);
        //println!("{num}");
    }

    return result;
}

fn print_map(input: &Vec<Vec<i32>>) {
    for row in input {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    }
    println!();
}
