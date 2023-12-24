use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let mut line_v: Vec<(f64, f64, f64, Vec<i64>, Vec<i64>)> = Vec::new();

    for line in contents.lines() {
        let (points, vels) = line.split_once('@').unwrap();
        println!("{} @ {}", points, vels);
        let vels = vels
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let point1 = points
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut point2 = point1.clone();
        point2[0] = point2[0] + vels[0];
        point2[1] = point2[1] + vels[1];
        point2[2] = point2[2] + vels[2];

        let (a, b, c) = find_equation(
            point1[0] as f64,
            point1[1] as f64,
            point2[0] as f64,
            point2[1] as f64,
        );
        line_v.push((a, b, c, point1, vels));
    }

    // let lower_bound = 7.;
    // let upper_bound = 27.;

    let lower_bound = 200000000000000.;
    let upper_bound = 400000000000000.;

    let mut num_intersects = 0;

    for comb in line_v.iter().combinations(2) {
        let (a1, b1, c1, points1, vels1) = comb[0];
        let (a2, b2, c2, points2, vels2) = comb[1];
        let (x, y) = find_intersection(*a1, *b1, *c1, *a2, *b2, *c2);

        if is_future(points1, vels1, x, y)
            && is_future(points2, vels2, x, y)
            && x > lower_bound
            && x < upper_bound
            && y > lower_bound
            && y < upper_bound
        {
            num_intersects += 1;
            println!("({},{})", x, y);
        }
    }
    println!("Part 1) {}", num_intersects)
}

fn is_future(points: &Vec<i64>, vels: &Vec<i64>, x: f64, y: f64) -> bool {
    let mut in_future = true;

    if vels[0] > 0 && (points[0] as f64) > x {
        in_future = false;
    }
    if vels[0] < 0 && (points[0] as f64) < x {
        in_future = false;
    }
    if vels[1] > 0 && (points[1] as f64) > y {
        in_future = false;
    }
    if vels[1] < 0 && (points[1] as f64) < y {
        in_future = false
    }
    in_future
}

fn find_intersection(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> (f64, f64) {
    let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
    let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);
    (x, y)
}

fn find_equation(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64, f64) {
    let a = y2 - y1;
    let b = x1 - x2;
    let c = y1 * (x2 - x1) - (y2 - y1) * x1;

    (a, b, c)
}
