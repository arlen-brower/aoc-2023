use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

const ROWS: usize = 13;
const COLS: usize = 13;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let mut row = 0;
    let mut col = 0;

    let mut grid: Vec<Vec<u32>> = vec![vec![0; ROWS]; COLS];
    for line in contents.lines() {
        col = 0;
        for ch in line.chars() {
            let int_ch = ch.to_digit(10).unwrap();
            grid[row][col] = int_ch;
            col += 1;
        }
        row += 1;
    }

    // print_map(&layout, ROWSows, COLSols);

    println!("---");
    let off = 5;
    let k = &grid[0 + off..3 + off];
    for r in k.iter() {
        for c in r[0 + off..3 + off].iter() {
            print!(" {:?}", c);
        }
        println!()
    }

    // print_grid(&grid, 13, 13);

    println!("---");
    println!("{}", bfs(&grid, (0, 0)));

    // match search(&grid, Vec::new(), (0, 0), 0, 's', 0) {
    //     (true, dist) => println!("{dist}"),
    //     _ => panic!(),
    // }
}

fn print_grid(grid: &Vec<Vec<u32>>, rows: usize, cols: usize) -> () {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!(" {}", grid[r][c]);
        }
        println!()
    }
}

fn search(
    grid: &Vec<Vec<u32>>,
    path: Vec<(usize, usize)>,
    pos: (usize, usize),
    dist: u32,
    dir: char,
    count: u32,
) -> (bool, u32) {
    // Path too long, not valid
    if count > 3 {
        return (false, dist);
    }
    // Path contains cycle, not valid
    if path.contains(&pos) {
        return (false, dist);
    }

    // At destination, valid path
    if pos.0 == ROWS - 1 && pos.1 == COLS - 1 {
        println!("{dist}");
        return (true, dist + grid[pos.0][pos.1]);
    }

    let r = pos.0;
    let c = pos.1;

    let mut n: Vec<(usize, usize, char, u32)> = Vec::new();
    if r + 1 < ROWS {
        let mut new_c = 0;
        if dir == '^' {
            new_c = count + 1;
        }
        if !path.contains(&(r + 1, c)) {
            n.push((r + 1, c, '^', new_c));
        }
    }
    // Down
    if r >= 1 {
        let mut new_c = 0;
        if dir == 'v' {
            new_c = count + 1;
        }
        if !path.contains(&(r - 1, c)) {
            n.push((r - 1, c, 'v', new_c));
        }
    }
    // Right
    if c + 1 < COLS {
        let mut new_c = 0;
        if dir == '>' {
            new_c = count + 1;
        }
        if !path.contains(&(r, c + 1)) {
            n.push((r, c + 1, '>', new_c));
        }
    }
    // Left
    if c >= 1 {
        let mut new_c = 0;
        if dir == '<' {
            new_c = count + 1;
        }
        if !path.contains(&(r, c - 1)) {
            n.push((r, c - 1, '<', new_c));
        }
    }

    let paths_v = n
        .iter()
        .map(|(r, c, d, count)| {
            let mut new_path = path.clone();
            new_path.push(pos);
            search(&grid, new_path, (*r, *c), grid[*r][*c] + dist, *d, *count)
        })
        .filter(|(valid, _)| *valid)
        .map(|(_, dist)| dist)
        .collect::<Vec<u32>>();

    if !paths_v.is_empty() {
        return (true, *paths_v.iter().min().unwrap());
    }

    (false, dist)
}

fn bfs(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    // let mut distances: Vec<Vec<u32>> = vec![vec![0; ROWS]; COLS];
    // let mut visited: Vec<Vec<bool>> = vec![vec![false; 13]; 13];
    let mut distances: HashMap<(usize, usize, char, u32), u32> = HashMap::new();
    let mut visited: HashSet<(usize, usize, char, u32)> = HashSet::new();

    let mut q: VecDeque<(usize, usize, char, u32)> = VecDeque::new();

    // distances[start.0][start.1] = 0;
    // visited[start.0][start.1] = true;
    distances.insert((start.0, start.1, 's', 0), 0);
    visited.insert((start.0, start.1, 's', 0));
    q.push_back((start.0, start.1, 's', 0));

    while !q.is_empty() {
        let (r, c, dir, count) = q.pop_front().unwrap();
        let mut n: Vec<(usize, usize, char, u32)> = Vec::new();

        // Up
        if r + 1 < ROWS {
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
        if c + 1 < COLS {
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
            if !visited.contains(&(nr, nc, nd, ncnt)) {
                visited.insert((nr, nc, nd, ncnt));
                // if !visited[nr][nc] {
                //     visited[nr][nc] = true;
                // distances[nr][nc] = distances[r][c] + grid[nr][nc];
                distances.insert(
                    (nr, nc, nd, ncnt),
                    grid[nr][nc] + distances.get(&(r, c, dir, count)).unwrap(),
                );
                q.push_back((nr, nc, nd, ncnt));
            }
        }
    }
    // print_grid(&distances, 13, 13);
    // distances[ROWS - 1][COLS - 1]

    for (r, c, d, cnt) in distances.keys() {
        if (*r, *c) == (ROWS - 1, COLS - 1) {
            println!(
                "{} {} : {}",
                r,
                c,
                distances.get(&(*r, *c, *d, *cnt)).unwrap()
            );
        }
    }

    *distances.get(&(ROWS - 1, COLS - 1, '>', 0)).unwrap()
}
