use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents = contents.replace("zero", "z0o");
    contents = contents.replace("one", "o1e");
    contents = contents.replace("two", "t2o");
    contents = contents.replace("three", "t3e");
    contents = contents.replace("four", "f4r");
    contents = contents.replace("five", "f5e");
    contents = contents.replace("six", "s6x");
    contents = contents.replace("seven", "s7n");
    contents = contents.replace("eight", "e8t");
    contents = contents.replace("nine", "n9e");

    let re = Regex::new(r"(?m)^[A-Za-z]*([0-9]).*([0-9])[A-Za-z]*$").unwrap();
    let re2 = Regex::new(r"(?m)^[A-Za-z]*([0-9])[A-Za-z]*$").unwrap();

    let mut result = 0;

    for (_, [first, second]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let tens = first.parse::<i32>().unwrap();
        let ones = second.parse::<i32>().unwrap();
        let combined = tens * 10 + ones;
        result = result + combined;
    }

    for (_, [digit]) in re2.captures_iter(&contents).map(|c| c.extract()) {
        let i_digit = digit.parse::<i32>().unwrap();
        let combined = i_digit * 10 + i_digit;
        result = result + combined;
    }
    println!("Result: {}", result);
}
