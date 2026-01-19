use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn num_list_to_big_num<T: Display>(nums: &[T]) -> Result<i128, std::num::ParseIntError> {
    let s: String = nums.iter()
        .map(|i| i.to_string())
        .collect();
    s.parse()
}