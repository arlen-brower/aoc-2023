use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    println!("graph G {{");
    for line in contents.lines() {
        let (node, adj) = line.split_once(':').unwrap();
        let adj = adj.trim();

        let mut adj_v: Vec<&str> = Vec::new();

        for n in adj.split(' ') {
            println!("  {} -- {};", node, n);
            adj_v.push(n);

            match graph.get_mut(n) {
                Some(adj_list) => {
                    adj_list.push(node);
                }
                None => {
                    graph.insert(n, vec![node]);
                }
            }
        }

        match graph.get_mut(node) {
            Some(adj_list) => {
                adj_list.append(&mut adj_v);
            }
            None => {
                graph.insert(node, adj_v);
            }
        }
    }
    println!("}}");
    bfs(&graph, "ttj");
    bfs(&graph, "rpd");
}

fn bfs(graph: &HashMap<&str, Vec<&str>>, start: &str) {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut q: VecDeque<&str> = VecDeque::new();

    visited.insert(start);
    q.push_back(start);

    while let Some(s) = q.pop_front() {
        let neighbours = graph.get(s).unwrap();
        for n in neighbours {
            if !visited.contains(n) {
                visited.insert(n);
                q.push_back(n);
            }
        }
    }
    eprintln!("Visited {} nodes", visited.len());
}
