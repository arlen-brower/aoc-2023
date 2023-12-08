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

    for line in nodes.lines() {
        let mut split_iter = line.split(char::is_whitespace);
        let k = split_iter.next().unwrap();
        let l = split_iter.next().unwrap();
        let r = split_iter.next().unwrap();
        node_map.insert(k, (l, r));
    }

    let mut cur_node: &str = "AAA";
    let mut steps = 0;

    while cur_node != "ZZZ" {
        println!("{cur_node}");
        let (l, r) = node_map[cur_node];
        cur_node = match instructions.next().unwrap() {
            'L' => l,
            'R' => r,
            _ => panic!(),
        };
        steps += 1;
    }

    println!("\nPart 1)\n{steps}");
}
