use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

const ROWS: usize = 13;
const COLS: usize = 13;
const MIN_LEN: u32 = 3;
const MAX_LEN: u32 = 10;

struct SearchPath {
    y: usize,
    x: usize,
    dir: char,
    count: u32,
    dist: u32,
    path: Vec<(usize, usize)>,
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; COLS]; ROWS];
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let int_ch = ch.to_digit(10).unwrap();
            grid[row][col] = int_ch;
        }
    }

    print_grid(&grid);

    println!("---");
    println!("{}", bfs(&grid, (0, 0)));
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn print_grid(grid: &[Vec<u32>]) {
    for row in grid.iter().take(ROWS) {
        for c in row.iter().take(COLS) {
            print!(" {}", c);
        }
        println!()
    }
}

fn bfs(grid: &[Vec<u32>], start: (usize, usize)) -> u32 {
    let mut visited: HashMap<(usize, usize, char, u32), u32> = HashMap::new();

    // let mut q: VecDeque<(usize, usize, char, u32, u32, Vec<(usize, usize)>)> = VecDeque::new();
    let mut q: VecDeque<SearchPath> = VecDeque::new();

    visited.insert((start.0, start.1, 's', 0), 0);
    q.push_back(SearchPath {
        y: start.0,
        x: start.1,
        dir: 's',
        count: 0,
        dist: 0,
        path: Vec::new(),
    });

    let mut v_dists: Vec<u32> = Vec::new();

    while !q.is_empty() {
        // let (r, c, dir, count, cur_dist, path) = q.pop_front().unwrap();
        let cur_path = q.pop_front().unwrap();

        let r = cur_path.y;
        let c = cur_path.x;
        let dir = cur_path.dir;
        let count = cur_path.count;
        let cur_dist = cur_path.dist;
        let path = cur_path.path;

        let n = get_neighbours(r, c, dir, count);

        let mut t_path = path.clone();
        t_path.push((r, c));

        if (r, c) == (ROWS - 1, COLS - 1) && count >= MIN_LEN {
            v_dists.push(cur_dist);
            continue;
        }

        for (nr, nc, nd, ncnt) in n {
            let new_dist = cur_dist + grid[nr][nc];
            if let Some(dist) = visited.get(&(nr, nc, nd, ncnt)) {
                if *dist < new_dist {
                    continue;
                }
            }

            visited.insert((nr, nc, nd, ncnt), new_dist);
            q.push_back(SearchPath {
                y: nr,
                x: nc,
                dir: nd,
                count: ncnt,
                dist: cur_dist + grid[nr][nc],
                path: t_path.clone(),
            });
        }
    }

    *v_dists.iter().min().unwrap()
}
fn get_neighbours(r: usize, c: usize, dir: char, count: u32) -> Vec<(usize, usize, char, u32)> {
    let mut n: Vec<(usize, usize, char, u32)> = Vec::new();
    // Up
    if r + 1 < ROWS && (dir == '^' || count >= MIN_LEN || dir == 's') {
        let mut new_c = 0;
        if dir == '^' {
            new_c = count + 1;
        }
        if new_c < MAX_LEN && dir != 'v' {
            n.push((r + 1, c, '^', new_c));
        }
    }
    // Down
    if r >= 1 && (dir == 'v' || count >= MIN_LEN || dir == 's') {
        let mut new_c = 0;
        if dir == 'v' {
            new_c = count + 1;
        }
        if new_c < MAX_LEN && dir != '^' {
            n.push((r - 1, c, 'v', new_c));
        }
    }
    // Right
    if c + 1 < COLS && (dir == '>' || count >= MIN_LEN || dir == 's') {
        let mut new_c = 0;
        if dir == '>' {
            new_c = count + 1;
        }
        if new_c < MAX_LEN && dir != '<' {
            n.push((r, c + 1, '>', new_c));
        }
    }
    // Left
    if c >= 1 && (dir == '<' || count >= MIN_LEN || dir == 's') {
        let mut new_c = 0;
        if dir == '<' {
            new_c = count + 1;
        }
        if new_c < MAX_LEN && dir != '>' {
            n.push((r, c - 1, '<', new_c));
        }
    }
    n
}
