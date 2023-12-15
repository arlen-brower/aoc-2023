use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let mut sum = 0;
    let mut boxes: Vec<Vec<&str>> = vec![Vec::new(); 256];
    for instruction in contents.split(',') {
        sum += hash(instruction);
        operation(instruction, &mut boxes);
    }

    let mut focus_power = 0;
    for i in 0..boxes.len() {
        if boxes[i].len() == 0 {
            continue;
        }

        for slot in 0..boxes[i].len() {
            let num: usize = boxes[i][slot]
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap();
            // println!(
            //     "{} * {} * {} = {}",
            //     i + 1,
            //     slot + 1,
            //     num,
            //     ((i + 1) * (slot + 1) * num)
            // );
            focus_power = focus_power + ((i + 1) * (slot + 1) * num);
        }
    }
    println!("P1) {sum}");
    println!("P2) {focus_power}");
}

fn operation<'a>(instruction: &'a str, boxes: &mut Vec<Vec<&'a str>>) -> () {
    let len = instruction.len();
    if instruction.contains('-') {
        let idx = hash(&instruction[..len - 1]);
        match boxes[idx]
            .iter()
            .position(|&x| &x[..x.len() - 2] == &instruction[..len - 1])
        {
            Some(i) => {
                boxes[idx].remove(i);
            }
            None => (),
        }
    } else if instruction.contains('=') {
        let idx = hash(&instruction[..len - 2]);

        match boxes[idx]
            .iter()
            .position(|&x| &x[..x.len() - 2] == &instruction[..len - 2])
        {
            Some(i) => {
                boxes[idx].remove(i);
                boxes[idx].insert(i, instruction);
            }
            None => boxes[idx].push(instruction),
        }
    } else {
        panic!();
    }
}

fn hash(instruction: &str) -> usize {
    let mut cur_val = 0;
    for ch in instruction.chars() {
        cur_val += ch as u32;
        cur_val *= 17;
        cur_val = cur_val % 256;
    }
    cur_val as usize
}
