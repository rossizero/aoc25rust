use std::{io};
use utils;

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let per_line: io::Result<Vec<Vec<i32>>> = lines
            .map(|line| {
                let s = line?;
                let digits: Vec<i32> = s
                    .trim()
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                    .collect();
                Ok(digits)
            })
            .collect();

        let a = per_line.unwrap().clone();
        let b = a.clone();
        let res1 = one(a);
        let res2 = two(b, 12);
        println!("result 1: {} 2: {}", res1, res2);
    }
}

fn one(matrix: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for line in matrix {
        let bb;
        let mut aa = line.iter().max().unwrap(); // last occurance - doesnt matter
        let pos = line.iter().position(|x| x == aa).unwrap(); //first occurance

        if pos == line.len() - 1 {
            bb = *aa;
            aa = line[..pos].iter().max().unwrap();
        } else {
            let (_, tail) = line.split_at(pos + 1);
            let tail: Vec<i32> = tail.to_vec();
            bb = *tail.iter().max().unwrap();
        }

        let tmp = format!("{}{}", aa, bb).parse::<i32>().unwrap();
        //println!("{tmp} {aa} {bb}");
        result += tmp;
    }
    return result;
}

fn two(matrix: Vec<Vec<i32>>, size: usize) -> i128 {
    let mut result: i128 = 0;

    for line in matrix {
        let mut s = size;
        let mut res = Vec::new();
        let mut working = line;
        
        while s > 0 {
            if s == working.len() {
                res.append(&mut working);
                break;
            }

            let pos = working.len().saturating_sub(s);
            let working_clone = working.clone();
            let (head, _) = working_clone.split_at(pos + 1);
            let head = head.to_vec();
            let (max, pos_max) = pos_first_max(&head);
            let right = working[pos_max + 1..].to_vec(); 

            working = right.to_vec();
            s -= 1;
            res.push(max);
        }

        let tmp = utils::num_list_to_big_num(&res).unwrap();
        //println!("{tmp}");
        result += tmp;
    }

    return result;
}

fn pos_first_max<T>(vector: &[T]) -> (T, usize)
where
    T: PartialOrd + Copy,
{
    let mut max: T = vector[0];
    let mut pos: usize = 0;

    for (i, &x) in vector.iter().enumerate() {
        if x > max {
            max = x;
            pos = i;
        }
    }
    return (max, pos);
}

// low: 167334875666958
fn two_wrong(matrix: Vec<Vec<i32>>, size: usize) -> i128 {
    let mut result = 0;

    for line in matrix {
        let mut pos = line.len().saturating_sub(size);
        let mut tail: Vec<i32> = line[pos..].to_vec();
        //println!("{:?}", line);
        //println!("{:?}", tail);

        while pos > 0 {
            pos -= 1;
            let min = tail.iter().min().unwrap();
            let pos_min = tail.iter().position(|x| x == min).unwrap();
            //println!("min {} pos_min {} line[pos]{}", min, pos_min, line[pos]);

            if line[pos] >= tail[0] {
                tail.remove(pos_min);
                tail.insert(0, line[pos]);
            }

            //println!("{:?}", tail);
        }

        let mut str = "".to_string();
        for i in tail {
            str += &i.to_string();
        }
        let tmp = str.parse::<i128>().unwrap();
        println!("{tmp}");
        result += tmp;
    }
    return result;
}
