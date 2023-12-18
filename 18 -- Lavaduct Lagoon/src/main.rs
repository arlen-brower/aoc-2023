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

    let mut poly_vec: Vec<(Point, Point)> = Vec::new();
    let mut pos = Point { x: 0, y: 0 };

    let mut border = 0;
    for line in contents.lines() {
        let (p1dir, rest) = line.split_once(' ').unwrap();
        let (dist_str, code) = rest.split_once(' ').unwrap();
        let p1dist = dist_str.parse::<u32>().unwrap();

        let code = code
            .trim_matches(|c| c == '(' || c == ')')
            .trim_start_matches('#');
        let (code, dir) = code.split_at(code.len() - 1);
        let code = i64::from_str_radix(code, 16).unwrap();

        let dist = code as i64;
        border += dist;
        let old_pos = pos;
        match dir {
            "R" | "0" => {
                pos = Point {
                    x: pos.x,
                    y: pos.y + dist,
                };
                poly_vec.push((old_pos, pos));
            }
            "L" | "2" => {
                pos = Point {
                    x: pos.x,
                    y: pos.y - dist,
                };
                poly_vec.push((old_pos, pos));
            }
            "U" | "3" => {
                pos = Point {
                    x: pos.x - dist,
                    y: pos.y,
                };
                poly_vec.push((old_pos, pos));
            }
            "D" | "1" => {
                pos = Point {
                    x: pos.x + dist,
                    y: pos.y,
                };
                poly_vec.push((old_pos, pos));
            }
            _ => panic!(),
        }
    }

    let mut sum = 0;

    for (point_one, point_two) in poly_vec {
        sum += point_one.x * point_two.y - point_two.x * point_one.y;
    }

    let area = (sum as f64 / 2.).abs();
    let interior = area - 0.5 * border as f64 + 1.;
    println!("{}", border as f64 + interior);
}
