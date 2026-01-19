use utils;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area(&self, p2: &Point) -> i64 {
        let width = (self.x - p2.x).abs() + 1;
        let height: i64 = (self.y - p2.y).abs() + 1;
        return width * height;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    p1: Point,
    p2: Point
}

impl Rect {
    fn area(&self) -> i64 {
        return self.p1.area(&self.p2);
    }

    fn width(&self) -> i64 {
        return (self.p1.x - self.p2.x).abs() + 1;
    }

    fn height(&self) -> i64 {
        return (self.p1.y - self.p2.y).abs() + 1;
    }

    fn x_min(&self) -> i64 { self.p1.x.min(self.p2.x) }
    fn x_max(&self) -> i64 { self.p1.x.max(self.p2.x) }
    fn y_min(&self) -> i64 { self.p1.y.min(self.p2.y) }
    fn y_max(&self) -> i64 { self.p1.y.max(self.p2.y) }

    fn inside(&self, rect: &Rect) -> bool {
        self.x_min() >= rect.x_min() &&
        self.x_max() <= rect.x_max() &&
        self.y_min() >= rect.y_min() &&
        self.y_max() <= rect.y_max()
    }

    fn contains(&self, point: &Point) -> bool {
        let x0 = self.p1.x.min(self.p2.x);
        let y0 = self.p1.y.min(self.p2.y);
    
        point.x >= x0 &&
        point.y >= y0 &&
        point.x <  x0 + self.width()  &&
        point.y <  y0 + self.height()
    }

    fn points(&self) -> Vec<Point> {
        let mut ret: Vec<Point> = Vec::new(); 
        let x = self.p1.x.min(self.p2.x);
        let y = self.p1.y.min(self.p2.y);

        for i in 0..self.width(){
            for j in 0..self.height() {
                ret.push(Point { x: (x + i), y: (y + j) });
            }
        }
        return ret;
    }

    fn points_edges(&self, sampling: i64) -> Vec<Point> {
        let mut edges = Vec::new();
    
        let x0 = self.p1.x.min(self.p2.x);
        let y0 = self.p1.y.min(self.p2.y);
        let w  = self.width();   // assume w > 0
        let h  = self.height();  // assume h > 0
    
        // Number of samples per edge (clamped to edge length)
        let sw = w.min(sampling).max(1);
        let sh = h.min(sampling).max(1);
    
        let step_x = if sw > 1 { w / (sw - 1) } else { 0 };
        let step_y = if sh > 1 { h / (sh - 1) } else { 0 };
    
        // Top & bottom edges
        for k in 0..sw {
            let x = x0 + k * step_x;
            edges.push(Point { x, y: y0 });           // top
            edges.push(Point { x, y: y0 + h - 1 });   // bottom
        }
    
        // Left & right edges
        for k in 0..sh {
            let y = y0 + k * step_y;
            edges.push(Point { x: x0,       y });     // left
            edges.push(Point { x: x0 + w - 1, y });  // right
        }
    
        edges
    }

}

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut points: Vec<Point> = Vec::new();
        for (i, mut line) in lines.enumerate() {
            if let Ok(ref mut l) = line {
                let nums:Vec<i64> = l
               .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();
                points.push(Point { x: (nums[0]), y: (nums[1]) })
            }
        }

        //println!("{:?}", points);
        println!("result 1: {} 2: {}", one(points.clone()), two(points.clone()));
    }
}

fn one(points: Vec<Point>) -> i64 {
    let mut area = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let tmp = points[i].area(&points[j]);
            //println!("{tmp}");
            area = tmp.max(area);
        }
    }
    return area;
}

fn two(points: Vec<Point>) -> i64 {
    let mut rects: Vec<Rect> = Vec::new();
    let mut area = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    //hollow polygon
    for (i, point) in points.iter().enumerate() {
        let next = (i+1) % (points.len());
        let point2 = points[next];
        let rect = Rect {p1: *point, p2: point2};
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
        rects.push(rect);
    }

    //rects are actually lines
    let b = rects.iter().any(|r| !(r.p1.x == r.p2.x || r.p1.y == r.p2.y));
    assert!(!b);

    /*for y in 0..max_y + 2 {
        for x in 0..max_x + 2 {
            let point = Point { x: x, y: y };
            if inside_poly_point(&rects, point) {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();
    }*/

    
    let mut wrong_rects: Vec<&Rect> = Vec::new();
    let mut all_rects: Vec<Rect> = Vec::new();
    let mut good_rects: Vec<&Rect> = Vec::new();
    
    for i in 0..points.len() {
        for j in i+1..points.len() { 
            let rect = Rect {p1: points[i], p2: points[j]};
            all_rects.push(rect);
        }
    }

    all_rects.sort_by_key(|r| r.area());

    for (i, rect) in all_rects.iter().enumerate() {
        //println!("{} {} {}/{}", points.len(), rect.area(), i, all_rects.len());
        if let Some(first) = wrong_rects.iter().find(|r| r.inside(&rect)) {
            //println!("skipped {} {} {}", rect.p1, rect.p2, rect.area());
            continue;
        }
        if rect.area() < area {
            continue;
        }

        let a = inside_poly(&rects, *rect, 50); // test out sampling ^^
        if a {
            //println!("{} {} {}", rect.p1, rect.p2, rect.area());
            area = rect.area().max(area);
            good_rects.push(rect);
        } else {
            wrong_rects.push(rect);
            //println!("wrong rects len {}", wrong_rects.len());
        }
    }
    //println!("current max {} len good {}", area, good_rects.len());
    return area;
}

fn inside_poly(poly: &Vec<Rect>, rect:Rect, sampling: i64) -> bool{
    let mut cont = false;
    for r in poly {
        let a = r.x_min() > rect.x_max();
        let b = r.x_max() < rect.x_min();

        let c = r.y_min() > rect.y_max();
        let d = r.y_max() < rect.y_min();

        if a || b || c || d {
            continue;
        }

        cont = true;
    }


    let vertical_lines = poly
    .iter()
    .filter(|r| r.p1.x == r.p2.x)
    .collect();

    if cont {
        let points = rect.points_edges(sampling);
        //println!("num points: {}", points.len());
        for (i, point) in points.iter().enumerate() {
            if !inside_poly_point(poly, &vertical_lines, *point) {
                //println!("stopped after {}", i);
                return false;
            }
        }
    }
    return true;
}

fn inside_poly_point(poly: &Vec<Rect>, vertical_lines: &Vec<&Rect>, point:Point) -> bool {
    if poly.iter().any(|r| r.contains(&point)) {
        return true;
    }

    let mut count = 0;
    for line in vertical_lines {
        if point.x <= line.p1.x && point.y >= line.y_min() && point.y < line.y_max(){
            count += 1;
        }
    }
    return count % 2 == 1;
}

fn inside_poly_point2(poly: &Vec<Rect>, point:Point, max_x: i64) -> bool {
    let mut interections = 0;
    let mut counter = 0;
    let on_edge = poly.iter().any(|r| r.contains(&point));
    let mut prev = on_edge;
    if !prev {
        for ray in point.x+1..max_x+2 {
            let point = Point {x: ray, y: point.y};
            let a = poly.iter().any(|r| r.contains(&point) );
            if !prev {
                if a {
                    interections += 1;
                }
            }
            if a {
                counter += 1;
            }
            prev = a;
        }
    }
    let result = (interections % 2 == 1 && interections > 0 && interections == counter) || on_edge;
    return result;
}