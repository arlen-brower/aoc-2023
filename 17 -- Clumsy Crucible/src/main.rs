use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

const ROWS: usize = 141;
const COLS: usize = 141;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let mut row = 0;
    let mut col = 0;
    let start = Instant::now();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; COLS]; ROWS];
    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            let int_ch = ch.to_digit(10).unwrap();
            grid[row][col] = int_ch;
            col += 1;
        }
        row += 1;
    }

    println!("---");
    println!("{}", bfs(&grid, (0, 0)));
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn print_grid(grid: &Vec<Vec<u32>>, rows: usize, cols: usize) -> () {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!(" {}", grid[r][c]);
        }
        println!()
    }
}

fn bfs(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let mut visited: HashMap<(usize, usize, char, u32), u32> = HashMap::new();

    let mut q: VecDeque<(usize, usize, char, u32, u32, Vec<(usize, usize)>)> = VecDeque::new();

    visited.insert((start.0, start.1, 's', 0), 0);
    q.push_back((start.0, start.1, 's', 0, 0, Vec::new()));

    let mut v_dists: Vec<u32> = Vec::new();

    while !q.is_empty() {
        let (r, c, dir, count, cur_dist, path) = q.pop_front().unwrap();
        let mut n: Vec<(usize, usize, char, u32)> = Vec::new();

        let mut t_path = path.clone();
        t_path.push((r, c));

        if (r, c) == (ROWS - 1, COLS - 1) && count >= 3 {
            v_dists.push(cur_dist);
            continue;
        }
        // Up
        if r + 1 < ROWS && (dir == '^' || count >= 3 || dir == 's') {
            let mut new_c = 0;
            if dir == '^' {
                new_c = count + 1;
            }
            if new_c < 10 && dir != 'v' {
                n.push((r + 1, c, '^', new_c));
            }
        }
        // Down
        if r >= 1 && (dir == 'v' || count >= 3 || dir == 's') {
            let mut new_c = 0;
            if dir == 'v' {
                new_c = count + 1;
            }
            if new_c < 10 && dir != '^' {
                n.push((r - 1, c, 'v', new_c));
            }
        }
        // Right
        if c + 1 < COLS && (dir == '>' || count >= 3 || dir == 's') {
            let mut new_c = 0;
            if dir == '>' {
                new_c = count + 1;
            }
            if new_c < 10 && dir != '<' {
                n.push((r, c + 1, '>', new_c));
            }
        }
        // Left
        if c >= 1 && (dir == '<' || count >= 3 || dir == 's') {
            let mut new_c = 0;
            if dir == '<' {
                new_c = count + 1;
            }
            if new_c < 10 && dir != '>' {
                n.push((r, c - 1, '<', new_c));
            }
        }

        for (nr, nc, nd, ncnt) in n {
            let new_dist = cur_dist + grid[nr][nc];
            match visited.get(&(nr, nc, nd, ncnt)) {
                Some(dist) => {
                    if *dist > new_dist {
                        visited.insert((nr, nc, nd, ncnt), new_dist);
                        q.push_back((nr, nc, nd, ncnt, cur_dist + grid[nr][nc], t_path.clone()));
                    }
                }
                None => {
                    visited.insert((nr, nc, nd, ncnt), new_dist);
                    q.push_back((nr, nc, nd, ncnt, cur_dist + grid[nr][nc], t_path.clone()));
                }
            }
        }
    }

    *v_dists.iter().min().unwrap()
}
