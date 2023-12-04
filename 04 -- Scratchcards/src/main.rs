use std::collections::HashMap;
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
    let mut card_map: HashMap<i32, i32> = HashMap::new();

    for i in 1..=games.len() {
        card_map.insert(i.try_into().unwrap(), 1);
    }

    for game in games {
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

        for gid in game_id + 1..=game_id + matches {
            let num_cards: &i32 = match card_map.get(&game_id) {
                Some(value) => &value,
                None => &1,
            };

            let copy_card = match card_map.get(&gid) {
                Some(value) => &value,
                None => &1,
            };

            card_map.insert(gid, copy_card + num_cards);
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

    println!();

    let mut copy_sum = 0;
    for (gid, n) in card_map {
        println!("Game {gid} : {n}");
        copy_sum = copy_sum + n;
    }

    println!("Part 1) Total Score = {score_sum}");
    println!("Part 2) Total Cards = {copy_sum}");
}
