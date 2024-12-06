use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let mut char_map: HashMap<(usize, usize), char> = HashMap::new();
    let mut lines = contents.lines();

    let start_pos = (0, lines.next().unwrap().find('.').unwrap());
    char_map.insert(start_pos, '.');

    let mut rows = 1; // Already read the first row
    let mut cols = 0;
    let mut last_line: &str = "";
    for (r, line) in lines.enumerate() {
        cols = 0;
        last_line = line;
        for (c, ch) in line.chars().enumerate() {
            if ch != '#' {
                char_map.insert((r + 1, c), ch);
            }
            cols += 1;
        }
        rows += 1;
    }

    let goal = (rows - 1, last_line.find('.').unwrap());

    println!("{} {}", rows, cols);

    // for r in 0..rows {
    //     for c in 0..cols {
    //         match char_map.get(&(r, c)) {
    //             Some(ch) => print!("{ch}"),
    //             None => print!("#"),
    //         }
    //     }
    //     println!();
    // }

    let distance = distances(&char_map, start_pos, goal);

    for r in 0..rows {
        for c in 0..cols {
            match char_map.get(&(r, c)) {
                Some(ch) => {
                    if distance.contains_key(&(r, c)) {
                        print!(
                            "{}",
                            distance
                                .get(&(r, c))
                                .unwrap()
                                .to_string()
                                .chars()
                                .nth(0)
                                .unwrap()
                        );
                    } else {
                        print!("{ch}");
                    }
                }
                None => print!("#"),
            }
        }
        println!();
    }

    println!("Start distance: {}", distance.get(&start_pos).unwrap());
    println!("Goal distance: {}", distance.get(&goal).unwrap());
    let max_dist = max(&distance, rows, cols);
    println!("Part 1) {} {:?}", max_dist.0, max_dist.1);
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

// fn color(distances: &HashMap<(usize, usize), usize>, pos: (usize, usize), maximum: usize) {
//     let distance = match distances.get(&pos) {
//         Some(d) => d,
//         None => return,
//     };
//     let intensity = (maximum as f32 - *distance as f32) / maximum as f32;
//     let dark = (255. * intensity) as i32;
//     let bright = 128 + (127. * intensity) as i32;
//     print!("\033[48;2;#{dark};#{bright};#{dark}m");
// }

fn max(
    distances: &HashMap<(usize, usize), usize>,
    rows: usize,
    cols: usize,
) -> (usize, (usize, usize)) {
    let mut max_distance = 0;
    let mut max_cell = (0, 1);

    for r in 0..rows {
        for c in 0..cols {
            match distances.get(&(r, c)) {
                Some(dist) => {
                    if *dist > max_distance {
                        max_distance = *dist;
                        max_cell = (r, c);
                    }
                }
                None => (),
            }
        }
    }

    (max_distance, max_cell)

    //
    //     def max
    // max_distance = 0
    // max_cell = @root
    // @cells.each do |cell, distance|
    // if distance > max_distance
    // max_cell = cell
    // max_distance = distance
    // end
    // end
    // [max_cell, max_distance]
    // end
}

#[derive(Clone, Eq, PartialEq)]
struct SearchPath {
    dist: usize,
    r: usize,
    c: usize,
    path: HashSet<(usize, usize)>,
}

impl Ord for SearchPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist
            .cmp(&other.dist)
            .then_with(|| self.r.cmp(&other.r))
            .then_with(|| self.c.cmp(&other.c))
    }
}

impl PartialOrd for SearchPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distances(
    grid: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    goal: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();

    // (distance, r, c)
    let mut q: BinaryHeap<SearchPath> = BinaryHeap::new();

    let mut start_path = HashSet::new();
    start_path.insert((start.0, start.1));
    distance.insert(start, 0);
    q.push(SearchPath {
        dist: 0,
        r: start.0,
        c: start.1,
        path: start_path,
    });

    let mut max_dist: Vec<usize> = Vec::new();

    while let Some(SearchPath { dist, r, c, path }) = q.pop() {
        if r == goal.0 && c == goal.1 {
            max_dist.push(dist);
            println!("{dist}");
            continue;
        }

        let neighbours = get_neighbours(&grid, (r, c), true);

        for (nr, nc) in neighbours {
            let next_dist = dist + 1;
            if !path.contains(&(nr, nc)) {
                let mut next_path = path.clone();
                next_path.insert((nr, nc));
                let next = SearchPath {
                    dist: next_dist,
                    r: nr,
                    c: nc,
                    path: next_path,
                };
                q.push(next);
                distance.insert((nr, nc), next_dist);
            }
        }
    }
    println!("Max: {}", max_dist.iter().max().unwrap());
    distance
}

fn get_neighbours(
    grid: &HashMap<(usize, usize), char>,
    pos: (usize, usize),
    part2: bool,
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    let cur = *(grid.get(&pos).unwrap());

    let u = (
        *(grid.get(&(pos.0 - 1, pos.1)).unwrap_or(&'#')),
        pos.0 - 1,
        pos.1,
    );
    let d = (
        *(grid.get(&(pos.0 + 1, pos.1)).unwrap_or(&'#')),
        pos.0 + 1,
        pos.1,
    );
    let l = (
        *(grid.get(&(pos.0, pos.1 - 1)).unwrap_or(&'#')),
        pos.0,
        pos.1 - 1,
    );
    let r = (
        *(grid.get(&(pos.0, pos.1 + 1)).unwrap_or(&'#')),
        pos.0,
        pos.1 + 1,
    );

    let dirs = vec![u, d, l, r];

    // No consraints on direction
    if cur == '.' || part2 {
        for (dir, r, c) in dirs {
            if dir != '#' {
                neighbours.push((r, c));
            }
        }
    } else {
        match cur {
            '>' => {
                if r.0 != '#' {
                    neighbours.push((r.1, r.2))
                }
            }
            '<' => {
                if l.0 != '#' {
                    neighbours.push((l.1, l.2))
                }
            }
            '^' => {
                if u.0 != '#' {
                    neighbours.push((u.1, u.2))
                }
            }
            'v' => {
                if d.0 != '#' {
                    neighbours.push((d.1, d.2))
                }
            }
            _ => panic!(),
        }
    }

    neighbours
}
