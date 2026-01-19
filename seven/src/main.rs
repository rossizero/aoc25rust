use utils;


fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut chars: Vec<Vec<char>> = Vec::new();
        for mut line in lines {
            if let Ok(ref mut l) = line {
                let l = l.replace("S", "|");
                chars.push(l.chars().collect());
            }
        }

        let (a,b) = one(chars.clone());
        println!("result 1: {} 2: {}", a, b)
    }
}

fn one(chars: Vec<Vec<char>>) -> (i32, i64) {
    let mut curr = chars[0].clone();
    let mut count:Vec<i64> = chars[0].iter().map(|&c| if c == '|' {1} else {0}).collect();

    //println!("{:?}", count);

    let mut result = 0;

    for (i, line) in chars[1..].iter().enumerate() {
        if i % 2 == 1 {
            
            let mut count_tmp:Vec<i64> = vec![0; count.len()];
            let pos: Vec<usize> = curr.iter().enumerate()
            .filter_map(|(id, &val)| if val == '|' {Some(id)} else {None })
            .collect();

            let pos2: Vec<usize> = line.iter().enumerate().
            filter_map(|(id, &val)| if pos.contains(&id) && val == '^' {Some(id)} else {None}).collect();
            
            let mut tmp = line.clone();
            let mut changed;

            for i in pos2.clone() {
                changed = false;
                if let Some(elem) = tmp.get_mut(i-1) {
                    *elem = '|';
                    count_tmp[i-1] += count[i];
                    changed = true;
                }
                if let Some(elem) = tmp.get_mut(i+1) {
                    *elem = '|';
                    count_tmp[i+1] += count[i];
                    changed = true;
                }
                if changed {
                    result += 1;
                }
            }

            for i in pos {
                if !pos2.contains(&i) {
                    if let Some(elem) = tmp.get_mut(i) {
                        *elem = '|';
                        count_tmp[i] += count[i];
                    }
                }
            }
            
            
            curr = tmp;
            count = count_tmp;
            //println!(" {:?} {}", count, count.iter().sum::<i64>());
        }
    }
    return (result, count.iter().sum::<i64>());
}
