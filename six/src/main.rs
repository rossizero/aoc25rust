use utils;

fn main() {
    let a = one();
    let b = two();
    println!("result 1: {} 2: {}", a, b)
}

fn one() -> i64 {
    let mut res1: i64 = 0;
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut nums: Vec<Vec<i64>> = Vec::new();
        for mut line in lines {
            if let Ok(ref mut l) = line {
                let num: Vec<i64> = l
                .split_whitespace()
                .map(|s| {
                    let a = s.parse::<i64>();
                    if let Ok(b) = a {
                        return b;
                    } 
                    if s == "*" {1} else {0}
                })
                .collect();
                nums.push(num);
            }
        }

        for i in 0..nums[0].len() {
            let mut column: Vec<i64> = Vec::new();
            for j in 0..nums.len() {
                column.push(nums[j][i]);
            }

            //println!("{:?}", column);
            if column[column.len() - 1] == 0 {
                let sum: i64 = column[..column.len()-1].iter().sum();
                res1 += sum;
            } else {
                let prod: i64 = column[..column.len()-1].iter().product();
                res1 += prod;
            }
        }
    }
    return res1;
}

fn two() -> i64 {
    let mut result: i64 = 0;
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut chars: Vec<Vec<char>> = Vec::new();

        for res_line in lines {
            if let Ok(line) = res_line {
                let tmp = line.chars().collect();
                //println!("{:?}", tmp);
                chars.push(tmp);
            }
        }
        
        let mut sum = true;
        let mut stuff: Vec<(Vec<i64>, bool)> = Vec::new();
        let mut curr: Vec<i64> = Vec::new();

        for i in 0..chars[0].len() {
            let mut string = String::new();

            for j in 0..chars.len() {
                let char:char = chars[j][i];
                if char =='*' || char == '+' {
                    stuff.push((curr, sum));
                    sum = char == '+';
                    curr = Vec::new();
                    break;
                }

                string.push(char);
            }
            //println!("{string}");
            if let Ok(num) = string.trim().parse::<i64>() {
                curr.push(num);
            }
        }
        stuff.push((curr, sum));

        //println!("{:?}", stuff);

        for (nums, sum) in stuff {
            if sum {
                result += nums.iter().sum::<i64>();
            } else {
                result += nums.iter().product::<i64>();
            }
        }

    }
    return result;
}