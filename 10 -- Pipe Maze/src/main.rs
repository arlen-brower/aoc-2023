use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

enum InOut {
    Outside,
    Inside,
    TopLine,
    BottomLine,
}

fn main() {
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

    let mut in_set: HashSet<(usize, usize)> = HashSet::new();

    let mut path_set: HashSet<(usize, usize)> = HashSet::new();
    let mut char_map: HashMap<(usize, usize), char> = HashMap::new();
    for r in 0..rows {
        for c in 0..cols {
            let cur: char = lines.get(r).unwrap().chars().nth(c).unwrap();
            if cur != '.' {
                path_set.insert((r, c));
            }
            char_map.insert((r, c), cur);
        }
    }

    use InOut::*;
    for r in 0..rows {
        // Every row starts from outside
        let mut in_out = Outside;

        for c in 0..cols {
            let cur_char = char_map.get(&(r, c)).unwrap();

            print!("{cur_char}");

            if path_set.contains(&(r, c)) {
                in_out = match (cur_char, &in_out) {
                    ('|', Outside) => Inside,
                    ('|', Inside) => Outside,
                    ('F', Outside) => TopLine,
                    ('F', Inside) => BottomLine,
                    ('L', Outside) => BottomLine,
                    ('L', Inside) => TopLine,
                    ('7', TopLine) => Outside,
                    ('7', BottomLine) => Inside,
                    ('J', TopLine) => Inside,
                    ('J', BottomLine) => Outside,
                    _ => in_out,
                }
            } else {
                match in_out {
                    Inside => {
                        in_set.insert((r, c));
                    }
                    Outside => (),
                    TopLine => {
                        println!("T{cur_char}");
                        panic!()
                    }
                    BottomLine => {
                        println!("B{cur_char}");
                        panic!()
                    }
                }
            }
        }
    }

    let mut in_count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if in_set.contains(&(r, c)) {
                print!(".");
                in_count += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }
    let area: i32 = (rows * cols).try_into().unwrap();
    println!("{in_count}");
    // println!("{}", area - out_count);
}
