use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    solve(contents);

    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn solve(contents: &str) {
    let mut layout: HashMap<(i64, i64), char> = HashMap::new();
    let mut steps: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut row = 0;
    let mut col = 0;

    let mut start: (i64, i64) = (0, 0);

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

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut q: VecDeque<(i64, i64, i64)> = VecDeque::new();

    q.push_back((start.0 as i64, start.1 as i64, 0));
    steps.insert((start.0 as i64, start.1 as i64, 0));

    let mut canvas_min_r: i64 = 0;
    let mut canvas_min_c: i64 = 0;
    let mut canvas_max_r: i64 = max_rows as i64;
    let mut canvas_max_c: i64 = max_cols as i64;

    let step_limit = 100;
    while let Some((r, c, step)) = q.pop_front() {
        let mut n: Vec<(i64, i64)> = Vec::new();

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

    let filtered_map: HashSet<(i64, i64)> = steps
        .iter()
        .filter(|(_, _, step)| step % 2 == 0)
        .map(|(r, c, _)| (*r, *c))
        .collect();

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
    println!("{}", filtered_map.len());
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn print_map(
    layout: &HashMap<(i64, i64), char>,
    visited: &HashSet<(i64, i64)>,
    min_rows: i64,
    min_cols: i64,
    max_rows: i64,
    max_cols: i64,
    mr: i64,
    mc: i64,
) -> () {
    for r in min_rows..max_rows {
        for c in min_cols..max_cols {
            match layout.get(&(modulo(r, mr), modulo(c, mc))) {
                Some(ch) => {
                    if visited.contains(&(r, c)) {
                        print!(".");
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
