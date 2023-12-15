use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let mut p1sum = 0;
    let mut p2sum = 0;

    for line in contents.lines() {
        let (block, num_str) = line.split_once(' ').unwrap();
        let mut nums: Vec<usize> = Vec::new();
        for n in num_str.split(',') {
            nums.push(n.parse().unwrap());
        }

        let b_string = block.to_string();

        let big_block = b_string.clone()
            + "?"
            + &b_string
            + "?"
            + &b_string
            + "?"
            + &b_string
            + "?"
            + &b_string;

        let mut big_nums: Vec<usize> = Vec::new();

        big_nums.extend_from_slice(&nums[..]);
        big_nums.extend_from_slice(&nums[..]);
        big_nums.extend_from_slice(&nums[..]);
        big_nums.extend_from_slice(&nums[..]);
        big_nums.extend_from_slice(&nums[..]);
        let mut cache: HashMap<(&str, &[usize]), usize> = HashMap::new();

        let p1count = solve(block.trim(), &nums[..], &mut cache);
        let p2count = solve(big_block.trim(), &big_nums[..], &mut cache);
        p1sum += p1count;
        p2sum += p2count;
    }
    println!("Part 1) {p1sum} arrangements");
    println!("Part 2) {p2sum} arrangements");

    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn octothorpe<'a>(
    block: &'a str,
    nums: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
    next_num: usize,
) -> usize {
    let mut x = next_num;
    while block.get(..x).is_none() {
        x -= 1;
    }

    let this_num: String = (&block[..x]).to_string().replace("?", "#");

    let mut test_string = String::new();

    for _i in 0..next_num {
        test_string.push('#');
    }

    if *this_num != test_string {
        return 0;
    }

    if block.len() == next_num {
        if nums.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    let peek_ch = block.chars().nth(next_num).unwrap();
    if peek_ch == '?' || peek_ch == '.' {
        return solve(&block[next_num + 1..], &nums[1..], cache);
    }

    return 0;
}

fn solve<'a>(
    block: &'a str,
    nums: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    match cache.get(&(block, nums)) {
        Some(x) => return *x,
        None => (),
    }

    if nums.len() == 0 {
        if block.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    if block.len() == 0 {
        return 0;
    }

    let next_ch = block.chars().nth(0).unwrap();
    let next_num = nums[0];

    let out = match next_ch {
        '#' => octothorpe(block, nums, cache, next_num),
        '.' => solve(&block[1..], nums, cache),
        '?' => solve(&block[1..], nums, cache) + octothorpe(block, nums, cache, next_num),
        _ => panic!(),
    };

    cache.insert((block, nums), out);
    out
}
