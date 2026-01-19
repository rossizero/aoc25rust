use utils;

fn main() {
    let mut x = 50;
    let mut n = 0;

    if let Ok(lines) = utils::read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let left = &line[..1] == "L";
            let num: i32 = line[1..].parse().unwrap();

            let inc = num / 100; // how often do we make a whole rotation
            let num2 = num - 100 * inc; //rest of the clicks left
            let tmp = x; // remember old x to not count 0's twice

            if left {
                x -= num2;
            } else {
                x += num2;
            }

            let mut counted = false;
            let m = ((x % 100) + 100) % 100;

            if tmp != 0 {
                // check if we rotated over or on top of the 0
                if x < 0 || x > 100 || m == 0 {
                    n += 1;
                    counted = true;
                }
            }

            //add all whole roations to counter
            n += inc;

            x = m;
            println!("{} {} {} {}", num, x, inc, counted);
            
        }
    }

    println!("result: {}", n);
}
