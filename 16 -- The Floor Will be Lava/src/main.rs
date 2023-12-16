use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let mut layout: HashMap<(usize, usize), char> = HashMap::new();

    let mut row = 0;
    let mut col = 0;

    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            layout.insert((row, col), ch);
            col += 1;
        }
        row += 1;
    }

    let max_rows = row;
    let max_cols = col;
    print_map(&layout, max_rows, max_cols);
    println!(
        "---\nP1) {}",
        find_energized((0, 0), '>', &layout, max_rows, max_cols)
    );

    let mut max = 0;
    for r in 0..max_rows {
        let lmax = find_energized((r, 0), '>', &layout, max_rows, max_cols);
        if lmax > max {
            max = lmax;
        }
    }
    for r in 0..max_rows {
        let lmax = find_energized((r, max_cols - 1), '<', &layout, max_rows, max_cols);
        if lmax > max {
            max = lmax;
        }
    }
    for c in 0..max_cols {
        let lmax = find_energized((0, c), 'v', &layout, max_rows, max_cols);
        if lmax > max {
            max = lmax;
        }
    }
    for c in 0..max_cols {
        let lmax = find_energized((max_rows - 1, c), '^', &layout, max_rows, max_cols);
        if lmax > max {
            max = lmax;
        }
    }
    println!("P2) {max}");
}

fn find_energized(
    start: (usize, usize),
    dir: char,
    layout: &HashMap<(usize, usize), char>,
    max_rows: usize,
    max_cols: usize,
) -> usize {
    let mut energized: HashSet<(usize, usize, char)> = HashSet::new();
    energize(start, dir, &layout, &mut energized, max_rows, max_cols);
    // println!("---");
    // print_energized(&energized, max_rows, max_cols);

    let mut flat_set: HashSet<(usize, usize)> = HashSet::new();

    for (r, c, _) in energized.iter() {
        flat_set.insert((*r, *c));
    }
    flat_set.len()
}

fn print_energized(layout: &HashSet<(usize, usize, char)>, max_rows: usize, max_cols: usize) -> () {
    for r in 0..max_rows {
        for c in 0..max_cols {
            if layout.contains(&(r, c, '>'))
                || layout.contains(&(r, c, '<'))
                || layout.contains(&(r, c, '^'))
                || layout.contains(&(r, c, 'v'))
            {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn energize(
    start: (usize, usize),
    dir: char,
    layout: &HashMap<(usize, usize), char>,
    visited: &mut HashSet<(usize, usize, char)>,
    max_rows: usize,
    max_cols: usize,
) -> () {
    let mut q: VecDeque<(usize, usize, char)> = VecDeque::new();
    match layout.get(&start) {
        Some('.') => {
            if !visited.contains(&(start.0, start.1, dir)) {
                q.push_back((start.0, start.1, dir));
            }
        }
        Some('/') => {
            let new_dir = match dir {
                '<' => 'v',
                '>' => '^',
                '^' => '>',
                'v' => '<',
                _ => panic!(),
            };
            if !visited.contains(&(start.0, start.1, new_dir)) {
                q.push_back((start.0, start.1, new_dir));
            }
        }
        Some('\\') => {
            let new_dir = match dir {
                '<' => '^',
                '>' => 'v',
                '^' => '<',
                'v' => '>',
                _ => panic!(),
            };
            if !visited.contains(&(start.0, start.1, new_dir)) {
                q.push_back((start.0, start.1, new_dir));
            }
        }
        Some('|') => {
            match dir {
                '<' | '>' => {
                    if !visited.contains(&(start.0, start.1, '^')) {
                        q.push_back((start.0, start.1, '^'));
                    }
                    if !visited.contains(&(start.0, start.1, 'v')) {
                        q.push_back((start.0, start.1, 'v'));
                    }
                }
                '^' | 'v' => {
                    if !visited.contains(&(start.0, start.1, dir)) {
                        q.push_back((start.0, start.1, dir));
                    }
                }
                _ => panic!(),
            };
        }
        Some('-') => {
            match dir {
                '<' | '>' => {
                    if !visited.contains(&(start.0, start.1, dir)) {
                        q.push_back((start.0, start.1, dir));
                    }
                }
                '^' | 'v' => {
                    if !visited.contains(&(start.0, start.1, dir)) {
                        q.push_back((start.0, start.1, '<'));
                    }
                    if !visited.contains(&(start.0, start.1, dir)) {
                        q.push_back((start.0, start.1, '>'));
                    }
                }
                _ => panic!(),
            };
        }
        Some(_) => panic!(),
        None => panic!(),
    }

    // q.push_back((start.0, start.1, dir));

    while !q.is_empty() {
        let (r, c, dir) = q.pop_front().unwrap();

        visited.insert((r, c, dir));
        let next_pos = match dir {
            '<' => match c.checked_sub(1) {
                Some(result) => (r, result),
                None => continue,
            },
            '>' => (r, c + 1),
            '^' => match r.checked_sub(1) {
                Some(result) => (result, c),
                None => continue,
            },
            'v' => (r + 1, c),
            _ => panic!(),
        };

        // If we are outside the map, we end that beam's path
        if next_pos.0 >= max_rows || next_pos.1 >= max_cols {
            continue;
        }

        match layout.get(&next_pos) {
            Some('.') => {
                if !visited.contains(&(next_pos.0, next_pos.1, dir)) {
                    q.push_back((next_pos.0, next_pos.1, dir));
                }
            }
            Some('/') => {
                let new_dir = match dir {
                    '<' => 'v',
                    '>' => '^',
                    '^' => '>',
                    'v' => '<',
                    _ => panic!(),
                };
                if !visited.contains(&(next_pos.0, next_pos.1, new_dir)) {
                    q.push_back((next_pos.0, next_pos.1, new_dir));
                }
            }
            Some('\\') => {
                let new_dir = match dir {
                    '<' => '^',
                    '>' => 'v',
                    '^' => '<',
                    'v' => '>',
                    _ => panic!(),
                };
                if !visited.contains(&(next_pos.0, next_pos.1, new_dir)) {
                    q.push_back((next_pos.0, next_pos.1, new_dir));
                }
            }
            Some('|') => {
                match dir {
                    '<' | '>' => {
                        if !visited.contains(&(next_pos.0, next_pos.1, '^')) {
                            q.push_back((next_pos.0, next_pos.1, '^'));
                        }
                        if !visited.contains(&(next_pos.0, next_pos.1, 'v')) {
                            q.push_back((next_pos.0, next_pos.1, 'v'));
                        }
                    }
                    '^' | 'v' => {
                        if !visited.contains(&(next_pos.0, next_pos.1, dir)) {
                            q.push_back((next_pos.0, next_pos.1, dir));
                        }
                    }
                    _ => panic!(),
                };
            }
            Some('-') => {
                match dir {
                    '<' | '>' => {
                        if !visited.contains(&(next_pos.0, next_pos.1, dir)) {
                            q.push_back((next_pos.0, next_pos.1, dir));
                        }
                    }
                    '^' | 'v' => {
                        if !visited.contains(&(next_pos.0, next_pos.1, dir)) {
                            q.push_back((next_pos.0, next_pos.1, '<'));
                        }
                        if !visited.contains(&(next_pos.0, next_pos.1, dir)) {
                            q.push_back((next_pos.0, next_pos.1, '>'));
                        }
                    }
                    _ => panic!(),
                };
            }
            Some(_) => panic!(),
            None => panic!(),
        }
    }
}

fn print_map(layout: &HashMap<(usize, usize), char>, max_rows: usize, max_cols: usize) -> () {
    for r in 0..max_rows {
        for c in 0..max_cols {
            match layout.get(&(r, c)) {
                Some(ch) => print!("{ch}"),
                None => panic!(),
            }
        }
        println!();
    }
}
