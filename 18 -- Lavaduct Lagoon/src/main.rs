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

    let dir_map: HashMap<&str, (i64, i64)> = HashMap::from([
        // Part Two:
        ("0", (0, 1)),
        ("1", (1, 0)),
        ("2", (0, -1)),
        ("3", (-1, 0)),
        // Part One:
        ("R", (0, 1)),
        ("D", (1, 0)),
        ("L", (0, -1)),
        ("U", (-1, 0)),
    ]);

    let start = Instant::now();

    solve(contents, &dir_map, parse_part1, "Part 1");
    solve(contents, &dir_map, parse_part2, "Part 2");

    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn parse_part1(line: &str) -> (&str, i64) {
    (&line[..1], line[2..4].trim().parse::<i64>().unwrap())
}

fn parse_part2(line: &str) -> (&str, i64) {
    let code = &line[line.len() - 9..line.len()];
    let dist = i64::from_str_radix(&code[2..code.len() - 2], 16).unwrap();
    let dir = &code[code.len() - 2..code.len() - 1];

    (dir, dist)
}

fn solve<F>(contents: &str, dir_map: &HashMap<&str, (i64, i64)>, parser: F, part_str: &str)
where
    F: Fn(&str) -> (&str, i64),
{
    let mut border = 0;
    let mut poly_vec: Vec<(Point, Point)> = Vec::new();
    let mut pos = Point { x: 0, y: 0 };
    for line in contents.lines() {
        let (dir, dist) = parser(line);

        let old_pos = pos;
        let (dx, dy) = dir_map.get(dir).unwrap();
        pos = Point {
            x: pos.x + dist * dx,
            y: pos.y + dist * dy,
        };
        poly_vec.push((old_pos, pos));
        border += dist;
    }

    // Shoelace
    let sum = poly_vec.iter().fold(0, |acc, (point_one, point_two)| {
        acc + (point_one.x * point_two.y - point_two.x * point_one.y)
    });
    let area = (sum as f64 / 2.).abs();

    // Pick's
    let interior = area - 0.5 * border as f64 + 1.;

    // Perimeter + Interior
    println!("{}: {}", part_str, border as f64 + interior);
}
