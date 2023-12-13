// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::env;
// use std::fs;
// use std::time::Instant;

fn main() {
    // let start = Instant::now();
    // let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    // let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = binding.trim_end();

    println!("{}", solve_block("???.###", &[1, 1, 3]));
    println!("{}", solve_block(".??..??...?##", &[1, 1, 3]));
    println!("{}", solve_block("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]));
    println!("{}", solve_block("????.#...#...", &[4, 1, 1]));
    println!("{}", solve_block("????.######..#####.", &[1, 6, 5]));
    println!("{}", solve_block("?###????????", &[3, 2, 1]));

    // println!("{}", solve_block("???????", &[2, 1]));
    // println!("{}", solve_block(".??..??...", &v[..]));
    // println!("{}", solve_block(".?#?.?..", &v[..]));
    // println!("{}", find_combinations(s, &v[..]));
}

fn solve_block(block: &str, nums: &[usize]) -> usize {
    if nums.len() == 0 {
        return 1;
    }

    if block.len() < nums[0] {
        // println!("{}", nums[0]);
        return 0;
    }

    let offset: usize;
    if nums.len() == 1 {
        offset = 0;
    } else {
        offset = 1;
    }
    let window: usize = nums[0];
    let len = block.len();
    let mut sum = 0;

    let mut j = 0;

    let mut cur_win = &block[j..j + window];
    if len == window && !cur_win.contains('.') {
        return 1;
    }

    while j < len - window - offset {
        cur_win = &block[j..j + window];
        while j < len - window && cur_win.contains('.') {
            cur_win = &block[j..j + window];
            j += 1;
        }
        if cur_win.contains('#') {
            while j < len - window && cur_win.chars().last().unwrap() == '#' {
                cur_win = &block[j..j + window];
                j += 1;
            }
        }
        if j > len - window - offset {
            j = len - window - offset;
        }

        sum += solve_block(&block[j + window + offset..], &nums[1..]);
        j += 1;
    }
    sum
}

// fn find_combinations(line: &str, mut nums: &mut [usize]) -> usize {
//     if nums.len() == 0 {
//         return 0;
//     }
//
//     let window: usize = nums[0];
//     let mut start: usize = 0;
//     let mut end: usize = window;
//
//     let blocks = line.split('.').collect::<Vec<&str>>();
//
//     let mut sum = 0;
//     for i in 0..blocks.len() {
//         let cur_block = blocks.get(i).unwrap();
//         let cur_len = cur_block.len();
//         if cur_len == 0 || cur_len < window {
//             continue;
//         }
//         if cur_block.contains('#') {
//             // Must match
//             let hash_count = cur_block
//                 .chars()
//                 .fold(0, |acc, ch| if ch == '#' { acc + 1 } else { acc });
//
//             if hash_count == window {
//                 nums = &mut nums[1..];
//                 continue;
//             }
//         } else {
//             sum += solve_block(cur_block, nums);
//         }
//     }
//
//     sum
// }
