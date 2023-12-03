use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let f = BufReader::new(File::open(file_path).unwrap());

    let arr: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c).collect()) 
        .collect();

    let rows = arr.len();
    let cols = arr[0].len();

    let mut parts: Vec<i32> = Vec::new();
    // I basically just made myself a mini-object for the Gears :P
    // (row, column, gear_id, part_number)
    // Gear ID was really just there in case there are part_numbers that are identical
    // ... but I don't think there were.
    let mut gears: Vec<(usize, usize, i32, i32)> = Vec::new();
    let mut gear_id: i32 = 0;
    let mut new_digit: bool = true;
    let mut has_symbol: bool = false;
    let mut left_idx = 0;
    let mut right_idx = 0;

    for r in 0..rows {
        let mut num_str = String::new();
        for c in 0..cols {
            
            if arr[r][c].is_digit(10) {

                if new_digit {
                    left_idx = c;
                    new_digit = false;
                }

                num_str.push(arr[r][c]);

                if c != cols-1 && arr[r][c+1].is_digit(10) {
                    // I thought I was going to do something here. I did not. :)
                }
                else {
                    // Digit parsed. Check for symbols.
                    right_idx = c; // Last known digit.
                    new_digit = true;

                    // Bounds checking stuff
                    let mut lbr = 0;
                    let mut ubr = rows-1;
                    let mut lbc = 0;
                    let mut ubc = cols-1;

                    if r != 0 {
                        lbr = r-1
                    }
                    if r != rows-1 {
                        ubr = r+1
                    }
                    if left_idx != 0{
                        lbc = left_idx-1;
                    }
                    if right_idx != cols-1{
                        ubc = right_idx+1;
                    }
                    // End Bounds Checking Stuff

                    // Neighbourhood search around digit perimeter
                    for search_rows in lbr..ubr+1 {
                        for search_cols in lbc..ubc+1{
                            let check_char = arr[search_rows][search_cols];
                            if !check_char.is_digit(10) && check_char != '.' {
                                has_symbol = true;
                                if check_char == '*' {
                                    gears.push((search_rows, search_cols, gear_id, num_str.parse().unwrap()));
                                    gear_id = gear_id + 1;
                                }
                            }
                        }
                    }

                    if has_symbol {
                        parts.push(num_str.parse().unwrap());
                    }
                    has_symbol = false;
                    num_str = String::new();
                    left_idx = 0;
                    right_idx = 0;

                }

            }// End Digit Parse
        } // End columns
    }// End rows
    

    // Messy business with me learning Rust collections!
    let mut gear_set = HashMap::new();
    let mut ratios: Vec<i32> = Vec::new();

    for (gr, gc, gid, part_no) in gears {
        println!("Gear {gid} -- {gr},{gc} : {part_no}");
        if gear_set.contains_key(&(gr,gc)){
            match gear_set.get(&(gr,gc)) {
                Some(pno ) => ratios.push(pno*part_no),
                None => println!("Oh no")
            }
        }
        else {
            gear_set.insert((gr,gc), part_no);
        }

    }

    let sum: i32 = parts.iter().sum();
    let ratio_sum: i32 = ratios.iter().sum();

    println!();
    println!("Part One: {sum}");
    println!("Part Two: {ratio_sum}");

}
