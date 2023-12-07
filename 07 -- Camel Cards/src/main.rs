use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

struct Hand {
    chars: Vec<char>,
    cards: HashMap<char, i32>,
    score: i32,
    number: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents = binding.trim_end();

    let v: Vec<&str> = contents.split('\n').collect();

    // Build a collection of Hands
    let mut hands: Vec<Hand> = Vec::new();
    for line in v {
        let temp: Vec<&str> = line.split(char::is_whitespace).collect();
        let card_str = temp[0];
        let num_str = temp[1];

        let mut card_map: HashMap<char, i32> = HashMap::new();
        for c in card_str.chars() {
            let count: &i32 = match card_map.get(&c) {
                Some(num) => num,
                None => &0,
            };
            card_map.insert(c, *count + 1);
        }

        let hand = Hand {
            chars: card_str.chars().collect::<Vec<char>>(),
            score: score_hand(&mut card_map),
            cards: card_map,
            number: num_str.parse().unwrap(),
        };

        hands.push(hand);
    }

    hands.iter().for_each(print_hand);

    hands.sort_by(hand_cmp);
    let mut winnings = 0;
    let mut rank = 1;
    for hand in hands {
        winnings = winnings + hand.number * rank;
        rank = rank + 1;
        print!("{} ", hand.number);
    }
    println!("\n Total winnings: {winnings}");
}

fn print_hand(hand: &Hand) {
    println!("---");
    print!("Cards: ");
    for c in &hand.chars {
        print!("{c} ")
    }
    println!("\nScore: {}", hand.score);
    print!("Freq: ");
    for c in hand.cards.keys() {
        print!("{c}:{}, ", hand.cards.get(&c).unwrap());
    }
    println!("\n---");
}

fn hand_cmp(a: &Hand, b: &Hand) -> Ordering {
    let ord = if a.score > b.score {
        Ordering::Greater
    } else if a.score < b.score {
        Ordering::Less
    } else {
        let mut i = 0;
        let mut t_ord = Ordering::Less;
        while i < 5 {
            if rank_char(&a.chars[i]) > rank_char(&b.chars[i]) {
                t_ord = Ordering::Greater;
                break;
            } else if rank_char(&a.chars[i]) < rank_char(&b.chars[i]) {
                break;
            }
            i = i + 1;
        }
        t_ord
    };
    ord
}

fn rank_char(c: &char) -> i32 {
    let rank = match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    };
    assert_ne!(rank, 0);
    rank
}

fn score_hand(hand: &mut HashMap<char, i32>) -> i32 {
    let j_val = match hand.remove(&'J') {
        Some(num) => num,
        None => 0,
    };
    let mut vals: Vec<i32> = hand.values().map(|i| *i).collect();

    vals.sort();
    println!("{:?}", vals);
    let max = vals.pop().unwrap_or(0);
    println!("{max}");
    vals.push(max + j_val);

    if vals.contains(&5) {
        println!("Woo!");
    }

    let score: i32 = if vals.contains(&5) {
        6 // Five of a kind
    } else if vals.contains(&4) {
        5 // Four of a kind
    } else if vals.contains(&3) && vals.contains(&2) {
        4 // Full house
    } else if vals.contains(&3) {
        3 // Three of a kind
    } else if vals.contains(&2) && vals.len() == 3 {
        2 // Two pair
    } else if vals.contains(&2) {
        1 // One pair
    } else if vals.contains(&1) {
        0 // High Card
    } else {
        -1 // Shouldn't happen.
    };

    assert_ne!(score, -1);
    score
}
