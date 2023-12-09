use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let mut lines: Vec<Vec<i64>> = contents
        .lines()
        .map(|l| {
            l.split(char::is_whitespace)
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let part_one: Vec<i64> = lines.iter().map(|l| find_next_num(&l[..])).collect();
    let p1sum: i64 = part_one.iter().sum();
    println!("Part One) {}", p1sum);

    lines
        .iter_mut()
        .for_each(|slice: &mut Vec<i64>| slice.reverse());

    let part_two: Vec<i64> = lines.iter().map(|l| find_next_num(&l[..])).collect();
    let p2sum: i64 = part_two.iter().sum();
    println!("Part Two) {}", p2sum);
}

fn find_next_num(nums: &[i64]) -> i64 {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }
    let mut d_nums: Vec<i64> = Vec::new();
    for i in 1..nums.len() {
        d_nums.push(nums[i] - nums[i - 1]);
    }

    nums.last().unwrap() + find_next_num(&d_nums[..])
}
