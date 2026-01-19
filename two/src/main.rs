use utils;


fn main() {
    let mut counter1 = 0;
    let mut counter2 = 0;
    if let Ok(lines) = utils::read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            for range in line.split(',') {
                let mut parts = range.split('-');
                if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                    counter1 += check1(first, second);
                    counter2 += check2(first, second);
                }
            }
        }
    }
    println!("result 1: {} 2: {}", counter1, counter2);
}

// build invalid ids using string concatenation 
fn check1(num1: &str, num2: &str) -> i64 {
    let mut res = 0;
    if let (Ok(a), Ok(b)) = (num1.parse::<i64>(), num2.parse::<i64>()) {
        //println!("{} {}", a, b);
        let mut half_num = "1";

        if num1.len() > 1 {
            let (half_num1,_) = num1.split_at(num1.len() / 2);
            half_num = half_num1;
        }
        let mut counter:i64 = half_num.parse().expect("...");
        let mut curr: String = format!("{}{}", counter, counter);

        while let Ok(curr_int) = curr.parse::<i64>(){
            if curr_int <= b {
                if curr_int >= a {
                    res += curr_int;
                    //println!("{}", curr_int);
                }
                counter += 1;
                curr = format!("{}{}", counter, counter);
            } else {
                break;
            }
        }
    }
    return res;
}

//more fancy string building -> also getting ugly because the same ids can be generated multiple times..
//so we have to remember which ones we already checked.. or thing about a better counter increase function
//also pretty stupid approach
//48631958998
fn check2(num1: &str, num2: &str) -> i64 {
    let mut res = 0;

    if let (Ok(a), Ok(b)) = (num1.parse::<i64>(), num2.parse::<i64>()) {
        //println!("{} {}", a, b);
        let mut store = Vec::new();

        for repeat in 2..=num2.chars().count() as i32 {
            let mut counter = 0;

            loop {
                counter += 1;
                let curr = format!("{}", counter).repeat(repeat as usize);
                //println!("repeat {} curr {} counter {}", repeat, curr, counter);
                let curr_int: i64 =  curr.trim().parse::<i64>().expect("invalid integer in curr");
                if store.contains(&curr_int) {
                    continue;
                }

                if curr_int <= b {
                    if curr_int >= a {
                        res += curr_int;
                        store.push(curr_int);
                        //println!("{}", curr_int); 
                    }
                } else {
                    break;
                }
            }
        } 
    }
    return res;
}