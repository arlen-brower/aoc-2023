use std::env;
use std::fs;
// use std::time::Instant;

fn main() {
    // let start = Instant::now();
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let mut sum_rows = 0;
    let mut sum_cols = 0;
    for valley in contents.split("\n\n") {
        let lines = valley.lines().collect::<Vec<&str>>();
        let (r, c) = solve_grid(lines);
        sum_rows += r;
        sum_cols += c;
        println!("{r} {c}");
    }
    let p1 = sum_cols + sum_rows * 100;
    println!("Part 2) {p1}");
}

fn solve_grid(lines: Vec<&str>) -> (usize, usize) {
    let mut by_rows = find_reflection_line(&lines);
    let mut by_cols = 0;
    if by_rows == 0 {
        let t_lines = transpose(lines);
        let strlines = t_lines.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        by_cols = find_reflection_line(&strlines);
    }
    (by_rows, by_cols)
}

fn transpose<'a>(lines: Vec<&str>) -> Vec<String> {
    let mut t_lines: Vec<String> = Vec::new();

    for col in 0..lines[0].len() {
        let mut line_builder = String::new();
        for line in &lines {
            line_builder.push(line.chars().nth(col).unwrap());
        }
        t_lines.push(line_builder.clone());
        // println!("{:?}", line_builder);
    }

    // t_lines
    t_lines
}

fn diff(a: &str, b: &str) -> usize {
    let mut count = 0;
    for i in 0..a.len() {
        let ch = a.chars().nth(i).unwrap();
        if ch != b.chars().nth(i).unwrap() {
            count += 1;
        }
    }
    count
}

fn find_reflection_line(lines: &Vec<&str>) -> usize {
    for i in 0..lines.len() - 1 {
        let mut mirror = false;
        let mut smudge_at: i32 = -1;
        let top = lines.get(i).unwrap();
        let bot = lines.get(i + 1).unwrap();
        if diff(top, bot) == 1 {
            mirror = true;
            smudge_at = 0;
        }
        if top == bot || smudge_at >= 0 {
            let mut n = 0;
            while n <= i && i + n + 1 < lines.len() {
                mirror = true;
                let t = lines.get(i - n).unwrap();
                let b = lines.get(i + n + 1).unwrap();

                if smudge_at < 0 && diff(t, b) == 1 {
                    smudge_at = n.try_into().unwrap();
                }

                if t != b && smudge_at != n.try_into().unwrap() {
                    mirror = false;
                    break;
                }
                n += 1;
            }
            if mirror && smudge_at >= 0 {
                return i + 1;
            }
        }
    }

    0
}
