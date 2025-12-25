use crate::utils::{self, parse_lines};
use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) {
    println!("=== Day 1 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in utils::parse_lines(input) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (from, rest) = line.split_once(": ").expect("bad line");
        let tos = rest.split_whitespace().map(String::from);

        map.entry(from.to_string()).or_default().extend(tos);
    }

    map
}

fn part1(input: &str) -> i32 {
    let map = parse(input);
    bfs_solution_part1(map)
}
fn bfs_solution_part1(map: HashMap<String, Vec<String>>) -> i32 {
    let mut total_paths = 0;
    let mut queue: VecDeque<String> = VecDeque::new();

    let start = String::from("you");

    queue.push_back(start.clone());

    while let Some(device) = queue.pop_front() {
        if let Some(neighbours) = map.get(&device) {
            for neigh in neighbours {
                if neigh == "out" {
                    total_paths += 1;
                } else {
                    queue.push_back(neigh.clone());
                }
            }
        }
    }
    total_paths
}

fn part2(input: &str) -> u64 {
    // NOTE: different example input for this part so won't work with current example
    let graph = parse(input);
    let mut memo: HashMap<(String, u8), u64> = HashMap::new();
    dfs_count("svr", 0, &graph, &mut memo)
}

// mask: 1 = seen_dac, 2 = seen_fft
fn dfs_count(
    node: &str,
    mut mask: u8,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    if node == "dac" {
        mask |= 1;
    }
    if node == "fft" {
        mask |= 2;
    }

    if node == "out" {
        return if mask == 3 { 1 } else { 0 };
    }

    let key = (node.to_string(), mask);
    if let Some(&v) = memo.get(&key) {
        return v;
    }

    let mut total = 0u64;
    if let Some(neighs) = graph.get(node) {
        for neigh in neighs {
            total += dfs_count(neigh, mask, graph, memo);
        }
    }

    memo.insert(key, total);
    total
}
