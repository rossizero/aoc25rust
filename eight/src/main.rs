use utils;
use std::{collections::HashMap, fmt, usize};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    group: usize
}

impl Point {
    fn new(x: f64, y: f64, z: f64, group: usize) -> Self {
        Self { x, y, z, group}
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}] g({})", self.x, self.y, self.z, self.group)
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut points: Vec<Point> = Vec::new();
        for (i,mut line) in lines.enumerate() {
            if let Ok(ref mut l) = line {
               let nums:Vec<f64> = l
               .split(',')
                .filter_map(|s| s.trim().parse::<f64>().ok())
                .collect();
                points.push(Point { x: nums[0], y: nums[1], z: nums[2], group:0});
            }
        }

        let a = one(points.clone(), 1000);
        let b = two(points.clone());
        println!("result 1: {} 2: {}", a, b)
    }
}

fn one(mut points: Vec<Point>, iterations: i32)  -> i32{
    let mut dists: Vec<(f64, (usize, usize))> = Vec::new();
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            let dist = p1.distance(p2);
            dists.push((dist, (i, j)));
        }
    }

    dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (i, entry) in dists.iter_mut().enumerate() {
        if i as i32 > iterations -1 {
            break;
        }
        let (_, (p1, p2)) = entry;
        let g1: usize = points[*p1].group;
        let g2: usize = points[*p2].group;

        //println!("{} {}: {}, {}: {}", i, p1, points[*p1], p2, points[*p2]);
        if g1 == g2 {
            if g2 == 0 {
                groups.insert(i+1, vec![*p1, *p2]);
                points[*p1].group = i+1;
                points[*p2].group = i+1;
            }
        } else {
            if g1 == 0 {
                groups.get_mut(&g2).unwrap().push(*p1);
                points[*p1].group = g2;
            } else if g2 == 0 {
                groups.get_mut(&g1).unwrap().push(*p2);
                points[*p2].group = g1;
            } else {
                //merge two groups
                let mut tmp: Vec<usize> = groups.remove(&g2).unwrap();
                let tmp2: &mut Vec<usize> = groups.get_mut(&g1).unwrap();
                //println!("{:?}", tmp);
                for i in tmp.clone() {
                    points[i].group = g1
                }
                tmp2.append(&mut tmp);
            }
        }

        points[*p2].group = points[*p1].group;
        //println!("{} {} {}", i, points[*p1], points[*p2]);
        //println!();
    }

    let mut entries: Vec<_> = groups.into_iter().collect();
    entries.sort_by_key(|(_, v)| v.len());
    entries.reverse();
    let a = entries[0].1.len() * entries[1].1.len() * entries[2].1.len();
    return a as i32;
}


fn two(mut points: Vec<Point>)  -> i64{
    let mut dists: Vec<(f64, (usize, usize))> = Vec::new();
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            let dist = p1.distance(p2);
            dists.push((dist, (i, j)));
        }
    }

    dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut a = true;
    let mut result = 0;

    while a {
        for (i, entry) in dists.iter_mut().enumerate() {
            let (_, (p1, p2)) = entry;
            let g1: usize = points[*p1].group;
            let g2: usize = points[*p2].group;

            if g1 == g2 {
                if g2 == 0 {
                    groups.insert(i+1, vec![*p1, *p2]);
                    points[*p1].group = i+1;
                    points[*p2].group = i+1;
                }
            } else {
                if g1 == 0 {
                    groups.get_mut(&g2).unwrap().push(*p1);
                    points[*p1].group = g2;
                } else if g2 == 0 {
                    groups.get_mut(&g1).unwrap().push(*p2);
                    points[*p2].group = g1;
                } else {
                    //merge two groups
                    let mut tmp: Vec<usize> = groups.remove(&g2).unwrap();
                    let tmp2: &mut Vec<usize> = groups.get_mut(&g1).unwrap();
                    for i in tmp.clone() {
                        points[i].group = g1
                    }
                    tmp2.append(&mut tmp);
                }
            }
    
            points[*p2].group = points[*p1].group;
            
            let check = points.iter().any(|p| p.group == 0);
            a = groups.len() > 1 || check;
            if !a {
                //println!("{} {}", points[*p1], points[*p2]);
                result = points[*p1].x as i64 * points[*p2].x as i64;
                break;
            }
        }
        
    }
    return result;
}
