use utils;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Shape {
    id: usize,
    width: u8,   // <= 64 if using u64
    height: u8,
    rows: [u64; 3], // fixed 3 rows for this puzzle
}

impl Shape {
    fn has(&self, x: u8, y: u8) -> bool {
        let row = self.rows[y as usize];
        (row >> x) & 1 == 1
    }
    fn size(&self) -> usize {
        let h = self.height as usize;
        let mut total = 0usize;
        for y in 0..h {
            total += self.rows[y].count_ones() as usize;
        }
        total
    }
    fn pretty_string(&self) -> String {
        let mut out = String::new();
        use std::fmt::Write;

        // Header line
        let _ = writeln!(
            &mut out,
            "id {} ({}x{}, size={}):",
            self.id,
            self.width,
            self.height,
            self.size()
        );

        // Each row
        let h = self.height as usize;
        let w = self.width as usize;
        for y in 0..h {
            let mut line = String::with_capacity(w);
            for x in 0..w {
                if self.has(x as u8, y as u8) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            let _ = writeln!(&mut out, "{line}");
        }

        out
    }

    fn print_pretty(&self) {
        print!("{}", self.pretty_string());
    }
}

fn build_shape_from_lines(id: usize, lines: &[String]) -> Shape {
    let height = lines.len();
    assert!(height <= 3, "Only up to 3 rows supported in Shape.rows");

    let width = lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    let mut rows_arr = [0u64; 3];

    for (y, line) in lines.iter().enumerate() {
        let mut mask: u64 = 0;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                mask |= 1 << x; // bit x set if '#'
            }
        }
        rows_arr[y] = mask;
    }

    Shape {
        id,
        width: width as u8,
        height: height as u8,
        rows: rows_arr,
    }
}

/// Parse the first part (shape definitions) and the second part (grid specs)
/// from the same file.
fn parse_input(
    input: &str,
) -> (
    HashMap<usize, Shape>,
    Vec<((usize, usize), Vec<usize>)>,
) {
    let mut shapes = HashMap::new();
    let mut grids: Vec<((usize, usize), Vec<usize>)> = Vec::new();

    let mut current_id: Option<usize> = None;
    let mut current_rows: Vec<String> = Vec::new();
    let mut in_shape_section = true;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        // If we see "NxM:" we switch to grid-section parsing
        if let Some(colon_pos) = line.find(':') {
            let left = &line[..colon_pos];

            // Detect "WxH" format like "4x4" or "12x5"
            if let Some(x_pos) = left.find('x') {
                // This is a grid line, e.g. "4x4: 0 0 0 0 2 0"
                in_shape_section = false;

                // If we were in the middle of a shape, finish it
                if let Some(id) = current_id.take() {
                    let shape = build_shape_from_lines(id, &current_rows);
                    shapes.insert(id, shape);
                    current_rows.clear();
                }

                // Parse width and height
                let w: usize = left[..x_pos].parse().expect("invalid grid width");
                let h: usize = left[x_pos + 1..].parse().expect("invalid grid height");

                // Parse ids after the colon
                let after_colon = &line[colon_pos + 1..].trim();
                let ids: Vec<usize> = if after_colon.is_empty() {
                    Vec::new()
                } else {
                    after_colon
                        .split_whitespace()
                        .map(|s| s.parse().expect("invalid shape id in grid line"))
                        .collect()
                };

                grids.push(((w, h), ids));
                continue;
            }
        }

        if in_shape_section {
            // Shape section: either "id:" or rows with '#' / '.'
            if let Some(idx) = line.find(':') {
                if let Some(id) = current_id.take() {
                    let shape = build_shape_from_lines(id, &current_rows);
                    shapes.insert(id, shape);
                    current_rows.clear();
                }

                let id: usize = line[..idx].parse().expect("Invalid shape id");
                current_id = Some(id);
            } else {
                current_rows.push(line.to_string());
            }
        } else {
        }
    }

    // Final shape, if the file ended in the shape section
    if let Some(id) = current_id {
        let shape = build_shape_from_lines(id, &current_rows);
        shapes.insert(id, shape);
    }

    (shapes, grids)
}

fn main() {
    // Read entire file into a string
    let mut input_string = String::new();
    if let Ok(lines) = utils::read_lines("./input.txt") {
        for line in lines.flatten() {
            input_string.push_str(&line);
            input_string.push('\n');
        }
    }

    let (shapes, grids) = parse_input(&input_string);
    println!("Shapes:");
    for (id, s) in &shapes {
        s.print_pretty();
    }

    println!("\nGrid specs as ((width, height), Vec<usize>):");
    for g in &grids {
        println!("{g:?}");
    }

    let res1 = one(&shapes, &grids);
    println!("result 1: {}", res1);
}

//421 too low
fn one(shapes: &HashMap<usize, Shape>, grids: &Vec<((usize, usize), Vec<usize>)>) -> i32 {
    let mut result = 0;

    for (wh, counts) in grids {
        let (width, height) = *wh;
        let amounts = counts;

        let loose_width  = width / 3;
        let loose_height = height / 3;
        let capacity_for_loose = loose_width * loose_height;

        let loose_presents: usize = amounts.iter().copied().sum();

        if loose_presents <= capacity_for_loose {
            //println!("works for sure!");
            result += 1;
            continue;
        }

        let available_area = width * height;

        let mut tight_placing = 0usize;
        for (i, &amount) in amounts.iter().enumerate() {
            let shape = shapes.get(&i).expect("shape id missing");
            let density = shape.size();
            tight_placing += amount * density;
        }

        if available_area < tight_placing {
            //println!("doesnt work for sure");
            continue;
        }

        println!("unclear: {wh:?} {amounts:?}");
    }

    result
}