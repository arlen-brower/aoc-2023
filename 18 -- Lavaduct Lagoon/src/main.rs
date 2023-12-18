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

    let mut dig_map: HashMap<(i32, i32), &str> = HashMap::new();

    let mut pos: (i32, i32) = (0, 0);

    for line in contents.lines() {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (dist_str, code) = rest.split_once(' ').unwrap();
        // println!("{} {} {}", dir, dist_str, code);
        let dist = dist_str.parse::<u32>().unwrap();

        for i in 0..dist {
            match dir {
                "R" => {
                    dig_map.insert(pos, code);
                    pos = (pos.0, pos.1 + 1);
                }
                "L" => {
                    dig_map.insert(pos, code);
                    pos = (pos.0, pos.1 - 1);
                }
                "U" => {
                    dig_map.insert(pos, code);
                    pos = (pos.0 - 1, pos.1);
                }
                "D" => {
                    dig_map.insert(pos, code);
                    pos = (pos.0 + 1, pos.1);
                }
                _ => panic!(),
            }
            if pos.0 > 500 {
                println!("{}", pos.0);
                panic!();
            }
        }
    }

    let mut min_r = 0;
    let mut min_c = 0;
    let mut max_r = 0;
    let mut max_c = 0;

    for (r, c) in dig_map.keys() {
        if *r < min_r {
            min_r = *r;
        }
        if *r > max_r {
            max_r = *r;
        }
        if *c < min_c {
            min_c = *c;
        }
        if *c > max_c {
            max_c = *c;
        }
    }

    // println!("{} {}\n{} {}", min_r, min_c, max_r, max_c);

    let inside: HashSet<(i32, i32)> = bfs(&dig_map, (1, 1));

    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if dig_map.contains_key(&(r, c)) || inside.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", dig_map.len() + inside.len());
}

fn bfs(dig_map: &HashMap<(i32, i32), &str>, start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    visited.insert(start);
    distances.insert(start, 0);
    q.push_back(start);

    while let Some((r, c)) = q.pop_front() {
        let mut n: Vec<(i32, i32)> = Vec::new();
        if dig_map.get(&(r + 1, c)).is_none() {
            n.push((r + 1, c));
        }
        if dig_map.get(&(r - 1, c)).is_none() {
            n.push((r - 1, c));
        }
        if dig_map.get(&(r, c + 1)).is_none() {
            n.push((r, c + 1));
        }
        if dig_map.get(&(r, c - 1)).is_none() {
            n.push((r, c - 1));
        }

        for neighbour in n {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                distances.insert(neighbour, distances.get(&(r, c)).unwrap() + 1);
                q.push_back(neighbour);
            }
        }
    }

    visited
}
