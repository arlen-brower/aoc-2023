use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

const DEBUG: bool = false;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let p1 = solve(contents, 64);
    let mut result;
    if p1 == 3503 {
        result = "✅"
    } else {
        result = "❌"
    }
    println!("Part 1) {} {}", p1, result);
    let x = 26501365;

    let p2: i128 = ((14275 * x * x) / 17161 + (27637 * x) / 17161 - 225714 / 17161) + 1;

    if p2 == 584211423220706 {
        result = "✅"
    } else {
        result = "❌"
    }
    println!("Part 2) {} {}", p2, result);
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn solve(contents: &str, step_limit: i128) -> i128 {
    let mut layout: HashMap<(i128, i128), char> = HashMap::new();
    let mut steps: HashSet<(i128, i128, i128)> = HashSet::new();
    let mut row = 0;
    let mut col = 0;

    let mut start: (i128, i128) = (0, 0);

    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            if ch == 'S' {
                start = (row, col);
                layout.insert((row, col), '.');
            } else {
                layout.insert((row, col), ch);
            }
            col += 1;
        }
        row += 1;
    }

    let max_rows = row;
    let max_cols = col;

    let mut visited: HashSet<(i128, i128)> = HashSet::new();
    let mut q: VecDeque<(i128, i128, i128)> = VecDeque::new();

    q.push_back((start.0 as i128, start.1 as i128, 0));
    steps.insert((start.0 as i128, start.1 as i128, 0));

    let mut canvas_min_r: i128 = 0;
    let mut canvas_min_c: i128 = 0;
    let mut canvas_max_r: i128 = max_rows as i128;
    let mut canvas_max_c: i128 = max_cols as i128;

    while let Some((r, c, step)) = q.pop_front() {
        let mut n: Vec<(i128, i128)> = Vec::new();

        if step > step_limit {
            continue;
        }

        if r < canvas_min_r {
            canvas_min_r = r;
        }
        if r > canvas_max_r {
            canvas_max_r = r;
        }
        if c < canvas_min_c {
            canvas_min_c = c;
        }
        if c > canvas_max_c {
            canvas_max_c = c;
        }

        // Neighbours
        if *layout
            .get(&(modulo(r - 1, max_rows), modulo(c, max_cols)))
            .unwrap()
            == '.'
        {
            //Up
            n.push((r - 1, c));
        }
        if *layout
            .get(&(modulo(r + 1, max_rows), modulo(c, max_cols)))
            .unwrap()
            == '.'
        {
            //Down
            n.push((r + 1, c));
        }
        if *layout
            .get(&(modulo(r, max_rows), modulo(c - 1, max_cols)))
            .unwrap()
            == '.'
        {
            //Left
            n.push((r, c - 1));
        }
        if *layout
            .get(&(modulo(r, max_rows), modulo(c + 1, max_cols)))
            .unwrap()
            == '.'
        {
            //Right
            n.push((r, c + 1));
        }

        for neighbour in n {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                q.push_back((neighbour.0, neighbour.1, step + 1));
                steps.insert((neighbour.0, neighbour.1, step + 1));
            }
        }
    }

    let even_odd = step_limit % 2;

    let filtered_map: HashSet<(i128, i128)> = steps
        .iter()
        .filter(|(_, _, step)| step % 2 == even_odd)
        .map(|(r, c, _)| (*r, *c))
        .collect();
    if DEBUG {
        print_map(
            &layout,
            &filtered_map,
            canvas_min_r,
            canvas_min_c,
            canvas_max_r,
            canvas_max_c,
            max_rows,
            max_cols,
        );
    }
    filtered_map.len() as i128
}

fn modulo(a: i128, b: i128) -> i128 {
    ((a % b) + b) % b
}

fn print_map(
    layout: &HashMap<(i128, i128), char>,
    visited: &HashSet<(i128, i128)>,
    min_rows: i128,
    min_cols: i128,
    max_rows: i128,
    max_cols: i128,
    mr: i128,
    mc: i128,
) -> () {
    for r in min_rows..=max_rows {
        for c in min_cols..=max_cols {
            match layout.get(&(modulo(r, mr), modulo(c, mc))) {
                Some(ch) => {
                    if visited.contains(&(r, c)) {
                        print!("O");
                    } else {
                        print!("{ch}");
                    }
                }
                None => panic!(),
            }
        }
        println!();
    }
}
