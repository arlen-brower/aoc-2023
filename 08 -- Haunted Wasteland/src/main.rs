use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input1".to_string());

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents = binding.trim_end();

    // let v: Vec<&str> = contents.split('\n').collect();

    let (instr_str, nodes) = contents.split_once("\n\n").unwrap();
    let mut instructions = instr_str.chars().cycle();
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starts: Vec<&str> = Vec::new();

    for line in nodes.lines() {
        let mut split_iter = line.split(char::is_whitespace);
        let k = split_iter.next().unwrap();
        let l = split_iter.next().unwrap();
        let r = split_iter.next().unwrap();
        node_map.insert(k, (l, r));

        if k.chars().nth(2).unwrap() == 'A' {
            starts.push(k);
            println!("{k}");
        }
    }

    let mut step_v: Vec<i64> = Vec::new();
    for n in starts {
        let mut cur_node: &str = n;
        let mut steps = 0;

        while cur_node.chars().nth(2).unwrap() != 'Z' {
            // println!("{cur_node}");
            let (l, r) = node_map[cur_node];
            cur_node = match instructions.next().unwrap() {
                'L' => l,
                'R' => r,
                _ => panic!(),
            };
            steps += 1;
        }
        step_v.push(steps);
        println!("\n{n}:\n{steps}");
    }
    println!("\nPart 2)\n{}", lcm(&step_v[..]));
}

fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
