use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents = binding.trim_end();

    let lines: Vec<&str> = contents.split('\n').collect();
    let games: Vec<&str> = lines
        .iter()
        .map(|l| match l.split_once(':') {
            Some((_, game)) => game,
            None => "ERR",
        })
        .collect();

    let mut game_id = 1;
    let mut score_sum = 0;
    for game in games {
        // println!("{game}");
        let (winning, owned_nums): (&str, &str) = match game.split_once('|') {
            Some((win, own)) => (win, own),
            None => ("ERR", "ERR"),
        };

        let win_ints: Vec<i32> = winning
            .trim()
            .split(char::is_whitespace)
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let own_ints: Vec<i32> = owned_nums
            .trim()
            .split(char::is_whitespace)
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let mut matches: i32 = 0;
        for n in win_ints {
            if own_ints.contains(&n) {
                matches = matches + 1;
            }
        }

        let base: i32 = 2;
        let mut score = 0;
        if matches >= 1 {
            matches = matches - 1;
            score = base.pow(matches.try_into().unwrap());
        }
        println!("Game {game_id}: {matches} matches worth {score} points");
        game_id = game_id + 1;
        score_sum = score_sum + score;
    }
    println!("Part 1) Total Score = {score_sum}");
}
