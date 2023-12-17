use std::cmp;
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

    let mut row = 0;
    let mut col = 0;

    let mut grid: Vec<Vec<u32>> = vec![vec![0; 13]; 13];
    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            let int_ch = ch.to_digit(10).unwrap();
            grid[row][col] = int_ch;
            col += 1;
        }
        row += 1;
    }

    let max_rows = row;
    let max_cols = col;
    // print_map(&layout, max_rows, max_cols);

    println!("---");
    let off = 5;
    let k = &grid[0 + off..3 + off];
    for r in k.iter() {
        for c in r[0 + off..3 + off].iter() {
            print!(" {:?}", c);
        }
        println!()
    }

    print_grid(&grid, 13, 13);

    println!("---");
    println!("{}", bfs(&grid, (0, 0)));
}

fn print_grid(grid: &Vec<Vec<u32>>, max_rows: usize, max_cols: usize) -> () {
    for r in 0..max_rows {
        for c in 0..max_cols {
            print!(" {}", grid[r][c]);
        }
        println!()
    }
}

fn bfs(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let mut distances: Vec<Vec<u32>> = vec![vec![0; 13]; 13];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 13]; 13];

    let mut q: VecDeque<(usize, usize, char, u32)> = VecDeque::new();

    let max_r = 13;
    let max_c = 13;

    distances[start.0][start.1] = 0;
    visited[start.0][start.1] = true;
    q.push_back((start.0, start.1, 's', 0));

    while !q.is_empty() {
        let (r, c, dir, count) = q.pop_front().unwrap();
        let mut n: Vec<(usize, usize, char, u32)> = Vec::new();

        // Up
        if r + 1 < max_r {
            let mut new_c = 0;
            if dir == '^' {
                new_c = count + 1;
            }
            if new_c <= 3 && dir != 'v' {
                n.push((r + 1, c, '^', new_c));
            }
        }
        // Down
        if r >= 1 {
            let mut new_c = 0;
            if dir == 'v' {
                new_c = count + 1;
            }
            if new_c <= 3 && dir != '^' {
                n.push((r - 1, c, 'v', new_c));
            }
        }
        // Right
        if c + 1 < max_c {
            let mut new_c = 0;
            if dir == '>' {
                new_c = count + 1;
            }
            if new_c <= 3 && dir != '<' {
                n.push((r, c + 1, '>', new_c));
            }
        }
        // Left
        if c >= 1 {
            let mut new_c = 0;
            if dir == '<' {
                new_c = count + 1;
            }
            if new_c <= 3 && dir != '>' {
                n.push((r, c - 1, '<', new_c));
            }
        }

        for (nr, nc, nd, ncnt) in n {
            if !visited[nr][nc] {
                visited[nr][nc] = true;
                distances[nr][nc] = distances[r][c] + grid[nr][nc];
                q.push_back((nr, nc, nd, ncnt));
            }
        }
    }
    print_grid(&distances, 13, 13);
    distances[12][12]
}
