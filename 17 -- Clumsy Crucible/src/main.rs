use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

const ROWS: usize = 141;
const COLS: usize = 141;
const MIN_LEN: u32 = 3;
const MAX_LEN: u32 = 10;

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchPath {
    r: usize,
    c: usize,
    dir: char,
    count: u32,
    dist: u32,
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    r: usize,
    c: usize,
    dir: char,
    count: u32,
}
impl Ord for SearchPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for SearchPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
            print!("{}", c);
        }
        println!()
    }
}

fn bfs(grid: &[Vec<u32>], start: (usize, usize)) -> u32 {
    let mut visited: HashMap<State, u32> = HashMap::with_capacity(917504);

    let mut q: BinaryHeap<SearchPath> = BinaryHeap::with_capacity(131072);

    visited.insert(
        State {
            r: start.0,
            c: start.1,
            dir: 's',
            count: 0,
        },
        0,
    );
    q.push(SearchPath {
        r: start.0,
        c: start.1,
        dir: 's',
        count: 0,
        dist: 0,
    });

    let mut v_dists: BinaryHeap<u32> = BinaryHeap::new();

    while let Some(SearchPath {
        r,
        c,
        dir,
        count,
        dist,
    }) = q.pop()
    {
        let n = get_neighbours(r, c, dir, count);

        if (r, c) == (ROWS - 1, COLS - 1) && count >= MIN_LEN {
            v_dists.push(dist);
            continue;
        }

        for (nr, nc, nd, ncnt) in n {
            let new_dist = dist + grid[nr][nc];
            if let Some(dist) = visited.get(&State {
                r: nr,
                c: nc,
                dir: nd,
                count: ncnt,
            }) {
                if *dist < new_dist {
                    continue;
                }
            }

            visited.insert(
                State {
                    r: nr,
                    c: nc,
                    dir: nd,
                    count: ncnt,
                },
                new_dist,
            );
            q.push(SearchPath {
                r: nr,
                c: nc,
                dir: nd,
                count: ncnt,
                dist: dist + grid[nr][nc],
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
