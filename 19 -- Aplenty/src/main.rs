use core::ops::Range;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Rule {
    attr: char,
    op: char,
    value: usize,
    result: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct PartsCombo {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    solve(contents);
}

fn solve(contents: &str) {
    let mut work_map: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts_vec: Vec<Part> = Vec::new();

    let (work_strs, parts) = contents.split_once("\n\n").unwrap();

    for line in work_strs.lines() {
        let (label, rest) = line.split_once('{').unwrap();
        let label_string = String::from(label);

        let mut work_strs: Vec<&str> = rest.split(',').collect();
        let last_rule = work_strs.pop().unwrap().trim_end_matches("}");

        let mut rules: Vec<Rule> = Vec::new();
        for rule in work_strs {
            let attr = rule.chars().nth(0).unwrap();
            let op = rule.chars().nth(1).unwrap();
            let (temp, res) = rule[2..].split_once(':').unwrap();
            let value = temp.parse::<usize>().unwrap();
            let result = res.to_string();

            rules.push(Rule {
                attr,
                op,
                value,
                result,
            })
        }
        rules.push(Rule {
            attr: 'l',
            op: 'x',
            value: 0,
            result: last_rule.to_string(),
        });
        work_map.insert(label_string, rules);
    }

    for part in parts.lines() {
        let xmas: Vec<usize> = part
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|x| x[2..].parse().unwrap())
            .collect();

        parts_vec.push(Part {
            x: *xmas.get(0).unwrap(),
            m: *xmas.get(1).unwrap(),
            a: *xmas.get(2).unwrap(),
            s: *xmas.get(3).unwrap(),
        })
    }

    let mut sum = 0;
    for (id, part) in parts_vec.iter().enumerate() {
        let mut temp_str: String;
        let mut result: &String = &String::from("in");
        while result != "A" && result != "R" {
            let workflow: &Vec<Rule> = work_map.get(result).unwrap();
            for rule in workflow {
                let value = match rule.attr {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    'l' => {
                        result = &rule.result;
                        break;
                    }
                    _ => panic!(),
                };

                result = match rule.op {
                    '>' => {
                        if value > rule.value {
                            &rule.result
                        } else {
                            temp_str = "no".to_string();
                            &temp_str
                        }
                    }
                    '<' => {
                        if value < rule.value {
                            &rule.result
                        } else {
                            temp_str = "no".to_string();
                            &temp_str
                        }
                    }
                    _ => panic!(),
                };
                if result != "no" {
                    break;
                }
            }
        }
        println!("Part {} : {}", id + 1, result);
        if *result == "A" {
            sum += part.x;
            sum += part.m;
            sum += part.a;
            sum += part.s;
        }
    }
    println!("Part 1) {}", sum);
    println!(
        "Part 2) {}",
        possibilities(
            &work_map,
            "in",
            PartsCombo {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001
            }
        )
    );
    println!("Expect) {}", 167409079868000);
}

fn possibilities(work_map: &HashMap<String, Vec<Rule>>, label: &str, parts: PartsCombo) -> usize {
    println!("{:?}", parts.s);
    if label == "A" {
        // println!(
        //     "{}",
        //     (parts.x.end - parts.x.start)
        //         * (parts.m.end - parts.m.start)
        //         * (parts.a.end - parts.a.start)
        //         * (parts.s.end - parts.s.start)
        // );
        return (parts.x.end - parts.x.start)
            * (parts.m.end - parts.m.start)
            * (parts.a.end - parts.a.start)
            * (parts.s.end - parts.s.start);
    }
    if label == "R" {
        return 0;
    }
    let workflow: &Vec<Rule> = work_map.get(label).unwrap();

    println!("{:?}", workflow);
    let mut cur_parts = parts.clone();
    let mut rule_combos = 0;
    for rule in workflow {
        match rule.attr {
            'x' => {
                let (tr, fa) = modify_combo(&parts.x, rule.value, rule.op);
                cur_parts = PartsCombo {
                    x: tr,
                    m: parts.m.clone(),
                    a: parts.a.clone(),
                    s: parts.s.clone(),
                };
                rule_combos += possibilities(work_map, &rule.result, cur_parts.clone());
                cur_parts = PartsCombo {
                    x: fa,
                    m: parts.m.clone(),
                    a: parts.a.clone(),
                    s: parts.s.clone(),
                };
            }
            'm' => {
                let (tr, fa) = modify_combo(&parts.m, rule.value, rule.op);
                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: tr,
                    a: parts.a.clone(),
                    s: parts.s.clone(),
                };
                rule_combos += possibilities(work_map, &rule.result, cur_parts.clone());

                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: fa,
                    a: parts.a.clone(),
                    s: parts.s.clone(),
                };
            }
            'a' => {
                let (tr, fa) = modify_combo(&parts.a, rule.value, rule.op);
                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: parts.m.clone(),
                    a: tr,
                    s: parts.s.clone(),
                };
                rule_combos += possibilities(work_map, &rule.result, cur_parts.clone());

                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: parts.m.clone(),
                    a: fa,
                    s: parts.s.clone(),
                };
            }
            's' => {
                let (tr, fa) = modify_combo(&parts.s, rule.value, rule.op);
                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: parts.m.clone(),
                    a: parts.a.clone(),
                    s: tr,
                };
                rule_combos += possibilities(work_map, &rule.result, cur_parts.clone());
                cur_parts = PartsCombo {
                    x: parts.x.clone(),
                    m: parts.m.clone(),
                    a: parts.a.clone(),
                    s: fa,
                };
            }
            'l' => rule_combos += possibilities(work_map, &rule.result, cur_parts.clone()),
            _ => panic!(),
        }
    }

    return rule_combos;
}

fn modify_combo(parts: &Range<usize>, num: usize, op: char) -> (Range<usize>, Range<usize>) {
    if op == '<' {
        (parts.start..num, num..parts.end)
    } else {
        (num + 1..parts.end, parts.start..num + 1)
    }
}
