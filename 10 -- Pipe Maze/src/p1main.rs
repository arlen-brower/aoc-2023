use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let start__time = Instant::now();
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let lines = contents.lines().collect::<Vec<&str>>();
    let rows = contents.lines().count();
    let cols = contents
        .lines()
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .len();

    let area: i32 = (rows * cols).try_into().unwrap();

    // println!("Total cells: {}", area);
    // println!("{}", area - do_maze(lines, rows, cols));
    do_maze(lines, rows, cols);
}
fn do_maze(lines: Vec<&str>, rows: usize, cols: usize) -> i32 {
    let mut char_map: HashMap<(usize, usize), char> = HashMap::new();
    let mut start: (usize, usize) = (1, 1);
    for r in 0..rows {
        for c in 0..cols {
            let cur: char = lines.get(r).unwrap().chars().nth(c).unwrap();
            if cur == 'S' {
                start = (r, c);
            }
            char_map.insert((r, c), cur);
        }
    }

    let mut up: (usize, usize);
    let mut down: (usize, usize);
    let mut right: (usize, usize);
    let mut left: (usize, usize);

    let mut upl: (usize, usize);
    let mut upr: (usize, usize);
    let mut dl: (usize, usize);
    let mut dr: (usize, usize);

    start = (1, 1);

    // TODO
    // char_map.insert(start, '|');

    let mut nodes: VecDeque<((usize, usize), u32)> = VecDeque::new();
    nodes.push_back((start, 0));

    let mut dists: Vec<u32> = Vec::new();
    let mut dist_map: HashMap<(usize, usize), char> = HashMap::new();
    let mut pipe_map: HashMap<(usize, usize), char> = HashMap::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    visited.push(start);

    while !nodes.is_empty() {
        let (cur_pos, cur_dist) = nodes.pop_front().unwrap();
        // println!("({},{})", cur_pos.0, cur_pos.1);
        up = (cur_pos.0 - 1, cur_pos.1);
        down = (cur_pos.0 + 1, cur_pos.1);
        right = (cur_pos.0, cur_pos.1 + 1);
        left = (cur_pos.0, cur_pos.1 - 1);

        upl = (cur_pos.0 - 1, cur_pos.1 - 1);
        upr = (cur_pos.0 - 1, cur_pos.1 + 1);
        dl = (cur_pos.0 + 1, cur_pos.1 - 1);
        dr = (cur_pos.0 + 1, cur_pos.1 + 1);

        dists.push(cur_dist);

        let c_up = char_map.get(&up).unwrap_or(&' ');
        let c_down = char_map.get(&down).unwrap_or(&' ');
        let c_right = char_map.get(&right).unwrap_or(&' ');
        let c_left = char_map.get(&left).unwrap_or(&' ');
        let c_cur = char_map.get(&cur_pos).unwrap();
        pipe_map.insert(cur_pos, *c_cur);
        dist_map.insert(cur_pos, cur_dist.to_string().chars().nth(0).unwrap());

        let c_upl = char_map.get(&upl).unwrap_or(&' ');
        let c_upr = char_map.get(&upr).unwrap_or(&' ');
        let c_dl = char_map.get(&dl).unwrap_or(&' ');
        let c_dr = char_map.get(&dr).unwrap_or(&' ');

        let mut neighbours: Vec<&char> = Vec::new();

        if can_squeeze(&char_map, cur_pos, *c_up, 'n') && !visited.contains(&up) {
            // if *c_up == '.' && !(visited.contains(&up)) {
            nodes.push_back((up, cur_dist + 1));
            visited.push(up);
            neighbours.push(c_up);
        }
        if can_squeeze(&char_map, cur_pos, *c_down, 's') && !visited.contains(&down) {
            // if *c_down == '.' && !(visited.contains(&down)) {
            nodes.push_back((down, cur_dist + 1));
            visited.push(down);
            neighbours.push(c_down);
        }
        if can_squeeze(&char_map, cur_pos, *c_right, 'e') && !visited.contains(&right) {
            // if *c_right == '.' && !(visited.contains(&right)) {
            nodes.push_back((right, cur_dist + 1));
            visited.push(right);
            neighbours.push(c_right);
        }
        if can_squeeze(&char_map, cur_pos, *c_left, 'w') && !visited.contains(&left) {
            // if *c_left == '.' && !(visited.contains(&left)) {
            nodes.push_back((left, cur_dist + 1));
            visited.push(left);
            neighbours.push(c_left);
        }
        if can_squeeze(&char_map, cur_pos, *c_upl, '7') && !visited.contains(&upl) {
            nodes.push_back((upl, cur_dist + 1));
            visited.push(upl);
        }
        if can_squeeze(&char_map, cur_pos, *c_upr, '9') && !visited.contains(&upr) {
            nodes.push_back((upr, cur_dist + 1));
            visited.push(upr);
        }
        if can_squeeze(&char_map, cur_pos, *c_dl, '1') && !visited.contains(&dl) {
            nodes.push_back((dl, cur_dist + 1));
            visited.push(dl);
        }
        if can_squeeze(&char_map, cur_pos, *c_dr, '3') && !visited.contains(&dr) {
            nodes.push_back((dr, cur_dist + 1));
            visited.push(dr);
        }
        // if connected(*c_cur, *c_up, 'n') && !visited.contains(&up) {
        //     // if *c_up == '.' && !(visited.contains(&up)) {
        //     nodes.push_back((up, cur_dist + 1));
        //     visited.push(up);
        //     neighbours.push(c_up);
        // }
        // if connected(*c_cur, *c_down, 's') && !visited.contains(&down) {
        //     // if *c_down == '.' && !(visited.contains(&down)) {
        //     nodes.push_back((down, cur_dist + 1));
        //     visited.push(down);
        //     neighbours.push(c_down);
        // }
        // if connected(*c_cur, *c_right, 'e') && !visited.contains(&right) {
        //     // if *c_right == '.' && !(visited.contains(&right)) {
        //     nodes.push_back((right, cur_dist + 1));
        //     visited.push(right);
        //     neighbours.push(c_right);
        // }
        // if connected(*c_cur, *c_left, 'w') && !visited.contains(&left) {
        //     // if *c_left == '.' && !(visited.contains(&left)) {
        //     nodes.push_back((left, cur_dist + 1));
        //     visited.push(left);
        //     neighbours.push(c_left);
        // }

        // Part 2 stuff
        // println!("({},{}): {:?}", cur_pos.0, cur_pos.1, neighbours);
    }

    // println!("{:?}", dists);

    // Really, ground count now
    let mut pipe_count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if dist_map.get(&(r, c)).unwrap_or(&'.').is_numeric()
                && *(char_map.get(&(r, c)).unwrap()) == '.'
            {
                pipe_count += 1;
                print!("{}", dist_map.get(&(r, c)).unwrap_or(&'.'));
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("{pipe_count}");
    // println!("Part 1) {}", dists.iter().max().unwrap());
    // println!("Elapsed: {}", start__time.elapsed().as_micros());
    pipe_count
}

fn can_squeeze(
    char_map: &HashMap<(usize, usize), char>,
    cur_pos: (usize, usize),
    dest: char,
    dir: char,
) -> bool {
    let mut ret_val = false;
    let mut up = (cur_pos.0 - 1, cur_pos.1);
    let mut down = (cur_pos.0 + 1, cur_pos.1);
    let mut right = (cur_pos.0, cur_pos.1 + 1);
    let mut left = (cur_pos.0, cur_pos.1 - 1);

    let mut upl = (cur_pos.0 - 1, cur_pos.1 - 1);
    let mut upr = (cur_pos.0 - 1, cur_pos.1 + 1);
    let mut dl = (cur_pos.0 + 1, cur_pos.1 - 1);
    let mut dr = (cur_pos.0 + 1, cur_pos.1 + 1);
    let c_up = char_map.get(&up).unwrap_or(&' ');
    let c_down = char_map.get(&down).unwrap_or(&' ');
    let c_right = char_map.get(&right).unwrap_or(&' ');
    let c_left = char_map.get(&left).unwrap_or(&' ');
    let c_cur = char_map.get(&cur_pos).unwrap();

    let c_upl = char_map.get(&upl).unwrap_or(&' ');
    let c_upr = char_map.get(&upr).unwrap_or(&' ');
    let c_dl = char_map.get(&dl).unwrap_or(&' ');
    let c_dr = char_map.get(&dr).unwrap_or(&' ');

    if dest == ' ' {
        return false;
    }

    if dest == '.' {
        return true;
    }

    // if dir == 'n' && dest != '-' {
    //     if dest == '|' || dest == 'L' || dest == 'F' {
    //         ret_val = true;
    //     }
    if dir == 's' && dest != '-' {
        if dest == '|' || dest == 'L' || dest == 'F' {
            ret_val = true;
        }
    } else if dir == 'e' && dest != '|' {
        if dest == '-' || dest == 'J' || dest == 'L' {
            ret_val = true;
        }
        // } else if dir == 'w' && dest != '|' {
        //     if dest == '-' || dest == 'F' || dest == '7' {
        //         ret_val = true;
        //     }
    }
    ret_val
}

fn connected(s: char, n: char, dir: char) -> bool {
    let mut ret_val = false;

    // Going North
    if dir == 'n' {
        if s == '.' || s == '|' || s == 'L' || s == 'J' {
            if n == '.' || n == '|' || n == '7' || n == 'F' {
                ret_val = true;
            }
        }
    }
    // Going East
    else if dir == 'e' {
        if s == '.' || s == '-' || s == 'L' || s == 'F' {
            if n == '.' || n == '-' || n == 'J' || n == '7' {
                ret_val = true;
            }
        }
    }
    // Going West
    else if dir == 'w' {
        if s == '.' || s == '-' || s == '7' || s == 'J' {
            if n == '.' || n == '-' || n == 'L' || n == 'F' {
                ret_val = true;
            }
        }
    }
    // Going South
    else if dir == 'n' {
        if s == '.' || s == '|' || s == 'F' || s == '7' {
            if n == '.' || n == '|' || n == 'L' || n == 'J' {
                ret_val = true;
            }
        }
    } else {
        panic!();
    }

    // println!("{s} to {n}, going {dir}: {ret_val}");
    ret_val
}
