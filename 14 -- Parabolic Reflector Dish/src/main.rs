use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end();

    let lines = contents.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();

    let mut round_set = HashSet::new();
    let mut cube_set = HashSet::new();

    for r in 0..rows {
        for c in 0..cols {
            let line = lines.get(r).unwrap();
            let ch = line.chars().nth(c);

            match ch {
                Some('O') => {
                    round_set.insert((r, c));
                }
                Some('#') => {
                    cube_set.insert((r, c));
                }
                Some('.') => (),
                Some(_) => panic!(),
                None => panic!(),
            }
        }
    }
    let mut sum = 0;
    let mut last_sum = 1;
    print_platform(&round_set, &cube_set, &rows, &cols);

    let start = Instant::now();
    for i in 1..=1000 {
        println!("After cycle");
        tilt_north(&mut round_set, &cube_set, &rows, &cols);
        tilt_west(&mut round_set, &cube_set, &rows, &cols);
        tilt_south(&mut round_set, &cube_set, &rows, &cols);
        tilt_east(&mut round_set, &cube_set, &rows, &cols);
        sum = round_set.iter().fold(0, |acc, (r, _)| acc + (rows - r));
        // print_platform(&round_set, &cube_set, &rows, &cols);
        println!("{sum},");
        println!("---");
    }
    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn tilt_north(
    round: &mut HashSet<(usize, usize)>,
    cubes: &HashSet<(usize, usize)>,
    rows: &usize,
    cols: &usize,
) -> () {
    for r in 0..*rows {
        for c in 0..*cols {
            match round.take(&(r, c)) {
                Some((round_r, round_c)) => {
                    let mut rolling = true;
                    let mut cur_r = round_r;

                    while rolling && cur_r > 0 {
                        cur_r -= 1;
                        if round.contains(&(cur_r, c)) || cubes.contains(&(cur_r, c)) {
                            cur_r += 1;
                            rolling = false;
                        }
                    }
                    round.insert((cur_r, round_c));
                }
                None => (),
            }
        } // end cols
    } // end rows
}

fn tilt_west(
    round: &mut HashSet<(usize, usize)>,
    cubes: &HashSet<(usize, usize)>,
    rows: &usize,
    cols: &usize,
) -> () {
    for c in 0..*cols {
        for r in 0..*rows {
            match round.take(&(r, c)) {
                Some((round_r, round_c)) => {
                    let mut rolling = true;
                    let mut cur_c = round_c;

                    while rolling && cur_c > 0 {
                        cur_c -= 1;
                        if round.contains(&(r, cur_c)) || cubes.contains(&(r, cur_c)) {
                            cur_c += 1;
                            rolling = false;
                        }
                    }
                    round.insert((round_r, cur_c));
                }
                None => (),
            }
        } // end cols
    } // end rows
}

fn tilt_east(
    round: &mut HashSet<(usize, usize)>,
    cubes: &HashSet<(usize, usize)>,
    rows: &usize,
    cols: &usize,
) -> () {
    for c in (0..*cols).into_iter().rev() {
        for r in 0..*rows {
            match round.take(&(r, c)) {
                Some((round_r, round_c)) => {
                    let mut rolling = true;
                    let mut cur_c = round_c;

                    while rolling && cur_c < *cols - 1 {
                        cur_c += 1;
                        if round.contains(&(r, cur_c)) || cubes.contains(&(r, cur_c)) {
                            cur_c -= 1;
                            rolling = false;
                        }
                    }
                    round.insert((round_r, cur_c));
                }
                None => (),
            }
        } // end cols
    } // end rows
}
fn tilt_south(
    round: &mut HashSet<(usize, usize)>,
    cubes: &HashSet<(usize, usize)>,
    rows: &usize,
    cols: &usize,
) -> () {
    for r in (0..*rows).into_iter().rev() {
        for c in 0..*cols {
            match round.take(&(r, c)) {
                Some((round_r, round_c)) => {
                    let mut rolling = true;
                    let mut cur_r = round_r;

                    while rolling && cur_r < *rows - 1 {
                        cur_r += 1;
                        if round.contains(&(cur_r, c)) || cubes.contains(&(cur_r, c)) {
                            cur_r -= 1;
                            rolling = false;
                        }
                    }
                    round.insert((cur_r, round_c));
                }
                None => (),
            }
        } // end cols
    } // end rows
}

fn print_platform(
    round: &HashSet<(usize, usize)>,
    cubes: &HashSet<(usize, usize)>,
    rows: &usize,
    cols: &usize,
) -> () {
    for r in 0..*rows {
        for c in 0..*cols {
            if round.contains(&(r, c)) {
                print!("O");
            } else if cubes.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("---");
}
