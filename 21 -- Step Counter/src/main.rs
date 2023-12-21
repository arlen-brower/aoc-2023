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
    let mut layout: HashMap<(usize, usize), char> = HashMap::new();
    let mut steps: HashSet<(usize, usize, u8)> = HashSet::new();
    let mut row = 0;
    let mut col = 0;

    let mut start: (usize, usize) = (0, 0);

    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            if ch == 'S' {
                start = (row, col);
                layout.insert((row, col), '.');
            }
            layout.insert((row, col), ch);
            col += 1;
        }
        row += 1;
    }

    let max_rows = row;
    let max_cols = col;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut q: VecDeque<(usize, usize, u8)> = VecDeque::new();

    q.push_back((start.0, start.1, 0));
    steps.insert((start.0, start.1, 0));

    let step_limit = 6;
    while let Some((r, c, step)) = q.pop_front() {
        let mut n: Vec<(usize, usize)> = Vec::new();

        if step > step_limit {
            continue;
        }
        // Neighbours
        if r > 0 && *layout.get(&(r - 1, c)).unwrap() == '.' {
            //Up
            n.push((r - 1, c));
        }
        if r < max_rows - 1 && *layout.get(&(r + 1, c)).unwrap() == '.' {
            //Down
            n.push((r + 1, c));
        }
        if c > 0 && *layout.get(&(r, c - 1)).unwrap() == '.' {
            //Left
            n.push((r, c - 1));
        }
        if c < max_cols - 1 && *layout.get(&(r, c + 1)).unwrap() == '.' {
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

    let filtered_map: HashSet<(usize, usize)> = steps
        .iter()
        .filter(|(_, _, step)| step % 2 == 0)
        .map(|(r, c, _)| (*r, *c))
        .collect();

    print_map(&layout, &filtered_map, max_rows, max_cols);
    println!("{}", filtered_map.len());
}

fn print_map(
    layout: &HashMap<(usize, usize), char>,
    visited: &HashSet<(usize, usize)>,
    max_rows: usize,
    max_cols: usize,
) -> () {
    for r in 0..max_rows {
        for c in 0..max_cols {
            match layout.get(&(r, c)) {
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
