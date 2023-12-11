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

    let mut galaxy_set: HashSet<(usize, usize)> = HashSet::new();
    let mut row: usize = 0;
    let mut col: usize;
    let mut max_rows: usize = 0;
    let mut max_cols: usize = 0;

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    for line in contents.lines() {
        let mut empty_row = true;
        row += 1;
        col = 0;
        for ch in line.chars() {
            col += 1;
            if ch == '#' {
                galaxy_set.insert((row, col));
                empty_row = false;
            }
        }
        if empty_row {
            empty_rows.push(row);
        }
        max_cols = col;
    }
    max_rows = row;

    for c in 1..=max_rows {
        let mut empty_col = true;
        for r in 1..=max_cols {
            if galaxy_set.contains(&(r, c)) {
                empty_col = false;
            }
        }
        if empty_col {
            empty_cols.push(c);
        }
    }
    println!("Empty Rows: {:?}", empty_rows);
    println!("Empty Cols: {:?}", empty_cols);

    let mut exp_set: HashSet<(usize, usize)> = HashSet::new();

    let growth = 1000000;

    for (r, c) in galaxy_set.iter() {
        let row_offset = (growth - 1)
            * empty_rows
                .iter()
                .fold(0, |acc, e| if r > e { acc + 1 } else { acc });
        let col_offset = (growth - 1)
            * empty_cols
                .iter()
                .fold(0, |acc, e| if c > e { acc + 1 } else { acc });

        exp_set.insert((r + row_offset, c + col_offset));
    }

    max_rows += empty_rows.len();
    max_cols += empty_cols.len();

    for r in 1..=max_rows {
        for c in 1..=max_cols {
            match exp_set.get(&(r, c)) {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }
        println!();
    }

    let mut dist_map: HashMap<((usize, usize), (usize, usize)), i64> = HashMap::new();

    for (y1, x1) in exp_set.iter() {
        for (y2, x2) in exp_set.iter() {
            dist_map.insert(((*y1, *x1), (*y2, *x2)), manhattan(*x1, *y1, *x2, *y2));
        }
    }

    println!("{} galaxies", exp_set.len());
    println!("{} pairs", dist_map.len());
    println!("{}", dist_map.values().sum::<i64>() / 2);
}

fn manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
    let ix1: i64 = x1.try_into().unwrap();
    let iy1: i64 = y1.try_into().unwrap();
    let ix2: i64 = x2.try_into().unwrap();
    let iy2: i64 = y2.try_into().unwrap();

    (ix1 - ix2).abs() + (iy1 - iy2).abs()
}
