use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let p1 = solve(contents);
    println!("Part 1) {}", p1);

    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

enum PowerState {
    On,
    Off,
}

use PowerState::*;

fn solve(contents: &str) -> i32 {
    // HashMap<label, (type, Vec<label>)
    let mut node_map: HashMap<&str, (char, Vec<&str>)> = HashMap::new();
    let mut conj_map: HashMap<&str, HashMap<&str, char>> = HashMap::new();
    let mut flip_map: HashMap<&str, PowerState> = HashMap::new();

    // Parse file into a map of nodes
    for line in contents.lines() {
        let (node, dest_str) = line.split_once(" -> ").unwrap();
        let label;
        let ch = match &node[0..1] {
            "%" => {
                label = &node[1..];
                flip_map.insert(label, Off);
                '%'
            }
            "&" => {
                // Assumes broadcaster never connects as input to Conj Node
                label = &node[1..];
                conj_map.insert(label, HashMap::new());
                '&'
            }
            "b" => {
                label = node;
                'b'
            }
            _ => panic!(),
        };
        node_map.insert(label, (ch, dest_str.split(',').map(|s| s.trim()).collect()));
    }

    let mut untyped: Vec<&str> = Vec::new();

    for line in contents.lines() {
        let (node, dest_str) = line.split_once(" -> ").unwrap();
        let conns: Vec<&str> = dest_str.split(',').map(|s| s.trim()).collect();

        for conn in conns {
            // Untyped node
            if !node_map.contains_key(conn) {
                untyped.push(conn);
            }
            // Input for a conjunction node
            if conj_map.contains_key(conn) {
                conj_map
                    .get_mut(conn)
                    .map(|cnode| cnode.insert(&node[1..], 'l'));
            }
        }
    }

    // Queue<source, destination, pulse-type>
    let mut q: VecDeque<(&str, &str, char)> = VecDeque::new();
    let mut high_count = 0;
    let mut low_count = 0;

    let mut p1;
    let mut p2 = 0;
    let mut i: i64 = 0;
    while p2 == 0 {
        //Push button
        q.push_back(("button", "broadcaster", 'l'));
        i += 1;
        if i == 1000 {
            p1 = low_count * high_count;
            println!("Part 1)\n {}", p1);
        }

        let mut rx_low = 0;
        while !q.is_empty() {
            let (src, dest, pulse_type) = q.pop_front().unwrap();

            if dest == "rx" && pulse_type == 'l' {
                rx_low += 1;
            }

            let (node_type, conns) = match node_map.get(dest) {
                Some(n) => n,
                None => {
                    // Untyped node
                    // Just add the signal and discard
                    if pulse_type == 'l' {
                        low_count += 1
                    } else {
                        high_count += 1
                    }
                    continue;
                }
            };

            if pulse_type == 'l' {
                low_count += 1;
            } else {
                high_count += 1;
            }

            match node_type {
                //Flipflop logic
                '%' => {
                    if pulse_type == 'l' {
                        let out_pulse = match flip_map.get(dest) {
                            Some(Off) => {
                                flip_map.insert(dest, On);
                                'h'
                            }
                            Some(On) => {
                                flip_map.insert(dest, Off);
                                'l'
                            }
                            None => panic!(),
                        };

                        for conn in conns {
                            q.push_back((dest, conn, out_pulse));
                        }
                    }
                }
                //Conjunction logic
                '&' => {
                    let out_pulse;
                    // Update memory for the input
                    conj_map
                        .get_mut(dest)
                        .map(|cnode| cnode.insert(src, pulse_type));

                    if conj_map[dest]
                        .values()
                        .map(|x| *x)
                        .collect::<Vec<char>>()
                        .contains(&'l')
                    {
                        out_pulse = 'h';
                    } else {
                        out_pulse = 'l';
                    }

                    for conn in conns {
                        q.push_back((dest, conn, out_pulse));
                    }
                }

                //Broadcaster logic
                'b' => {
                    for conn in conns {
                        q.push_back((dest, conn, pulse_type));
                    }
                }
                'u' => {
                    continue;
                }
                _ => panic!(),
            }
        }
        if rx_low == 1 {
            p2 = i;
        }
    }
    println!("Part 2)\n {}", p2);
    // println!("Low: {}\nHigh: {}", low_count, high_count);
    low_count * high_count
}
