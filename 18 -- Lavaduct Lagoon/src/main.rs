use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let dir_map: HashMap<&str, (i64, i64)> =
        HashMap::from([("0", (0, 1)), ("1", (1, 0)), ("2", (0, -1)), ("3", (-1, 0))]);

    let start = Instant::now();
    let mut poly_vec: Vec<(Point, Point)> = Vec::new();
    let mut pos = Point { x: 0, y: 0 };

    let mut border = 0;
    for line in contents.lines() {
        let code = &line[line.len() - 9..line.len()];
        let dist = i64::from_str_radix(&code[2..code.len() - 2], 16).unwrap();
        let dir = &code[code.len() - 2..code.len() - 1];

        let old_pos = pos;
        let (dx, dy) = dir_map.get(dir).unwrap();
        pos = Point {
            x: pos.x + dist * dx,
            y: pos.y + dist * dy,
        };
        poly_vec.push((old_pos, pos));
        border += dist;
    }

    let mut sum = 0;

    for (point_one, point_two) in poly_vec {
        sum += point_one.x * point_two.y - point_two.x * point_one.y;
    }

    let area = (sum as f64 / 2.).abs();
    let interior = area - 0.5 * border as f64 + 1.;
    println!("{}", border as f64 + interior);
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}
