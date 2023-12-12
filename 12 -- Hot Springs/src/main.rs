use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let v: Vec<usize> = vec![2, 1];

    let s = "???.###";

    println!("{}", solve_block("???????", &v[..]));
    println!("{}", solve_block(".??..??...", &v[..]));
    println!("{}", solve_block(".#?....?.?", &v[..]));
    // println!("{}", find_combinations(s, &v[..]));
}

fn solve_block(block: &str, nums: &[usize]) -> usize {
    if nums.len() == 0 {
        return 1;
    }

    if block.len() < nums[0] {
        return 0;
    }

    let window: usize = nums[0];
    let len = block.len();
    let mut sum = 0;

    for j in 0..len - window {
        let mut cur_win = &block[j..j + window];
        if cur_win.contains('.') {
            continue;
        }
        if cur_win.contains('#') {
            let mut grow = 1;
            let mut hash_count = cur_win
                .chars()
                .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc });
            while hash_count < window && cur_win.chars().last().unwrap() == '#' {
                cur_win = match block.get(j..j + window + grow) {
                    Some(x) => x,
                    None => break,
                };
                hash_count = cur_win
                    .chars()
                    .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc });

                grow += 1;
            }
        }
        sum += solve_block(&block[j + window + 1..], &nums[1..])
    }
    sum
}

fn find_combinations(line: &str, mut nums: &mut [usize]) -> usize {
    if nums.len() == 0 {
        return 0;
    }

    let window: usize = nums[0];
    let mut start: usize = 0;
    let mut end: usize = window;

    let blocks = line.split('.').collect::<Vec<&str>>();

    let mut sum = 0;
    for i in 0..blocks.len() {
        let cur_block = blocks.get(i).unwrap();
        let cur_len = cur_block.len();
        if cur_len == 0 || cur_len < window {
            continue;
        }
        if cur_block.contains('#') {
            // Must match
            let hash_count = cur_block
                .chars()
                .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc });

            if hash_count == window {
                nums = &mut nums[1..];
                continue;
            }
        } else {
            sum += solve_block(cur_block, nums);
        }
    }

    sum
}
