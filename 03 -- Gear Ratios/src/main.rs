use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let f = BufReader::new(File::open(file_path).unwrap());

    let arr: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c).collect())
        .collect();

    for line in arr {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}
