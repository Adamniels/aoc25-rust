use crate::utils;
use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) {
    println!("=== Day 10 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

/* -------------------- Parsing -------------------- */

fn parse_input(input: &str) -> Vec<(usize, u64, Vec<u64>, Vec<u16>)> {
    let mut out: Vec<(usize, u64, Vec<u64>, Vec<u16>)> = vec![];

    for line in utils::parse_lines(input) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (num_lights, target_lights, buttons, joltage) = parse_line(line);
        out.push((num_lights, target_lights, buttons, joltage));
    }

    out
}

fn parse_line(line: &str) -> (usize, u64, Vec<u64>, Vec<u16>) {
    // --- diagram [ ... ] ---
    let lb = line.find('[').expect("Missing '['");
    let rb = line[lb + 1..].find(']').expect("Missing ']'") + (lb + 1);
    let diagram = &line[lb + 1..rb];
    let num_lights = diagram.len();

    let mut target_lights: u64 = 0;
    for (i, ch) in diagram.chars().enumerate() {
        if ch == '#' {
            target_lights |= 1u64 << i;
        }
    }

    // --- buttons ( ... ) ---
    let mut buttons: Vec<u64> = Vec::new();
    let mut idx = rb + 1;

    while let Some(open_rel) = line[idx..].find('(') {
        let open = idx + open_rel;

        // stop if next '{' appears before next '('
        if let Some(brace_rel) = line[idx..].find('{') {
            let brace = idx + brace_rel;
            if brace < open {
                break;
            }
        }

        let close_rel = line[open + 1..].find(')').expect("Missing ')'");
        let close = open + 1 + close_rel;

        let inside = line[open + 1..close].trim();

        let mut mask: u64 = 0;
        if !inside.is_empty() {
            for part in inside.split(',') {
                let p = part.trim();
                if p.is_empty() {
                    continue;
                }
                let bit: usize = p.parse().expect("Bad index in button");
                mask |= 1u64 << bit;
            }
        }

        buttons.push(mask);
        idx = close + 1;
    }

    // --- joltage { ... } ---
    let mut joltage: Vec<u16> = Vec::new();
    if let Some(ob) = line.find('{') {
        let cb = line[ob + 1..].find('}').expect("Missing '}'") + (ob + 1);
        let inside = line[ob + 1..cb].trim();
        if !inside.is_empty() {
            joltage = inside
                .split(',')
                .map(|s| s.trim().parse::<u16>().expect("Bad joltage number"))
                .collect();
        }
    }

    (num_lights, target_lights, buttons, joltage)
}

/* -------------------- Part 1 (BFS on bitmask) -------------------- */

fn min_presses_lights(num_lights: usize, target: u64, buttons: &[u64]) -> u32 {
    let max_state = 1usize << num_lights;
    let mut dist = vec![u32::MAX; max_state];
    let mut queue = VecDeque::new();

    dist[0] = 0;
    queue.push_back(0usize);

    while let Some(state) = queue.pop_front() {
        let d = dist[state];

        if state as u64 == target {
            return d;
        }

        for &button in buttons {
            let next = state ^ (button as usize);
            if dist[next] == u32::MAX {
                dist[next] = d + 1;
                queue.push_back(next);
            }
        }
    }

    panic!("No solution found for Part 1 (unexpected)");
}

fn part1(input: &str) -> i32 {
    let mut result: i64 = 0;
    for (num_lights, target, buttons, _joltage) in parse_input(input) {
        result += min_presses_lights(num_lights, target, &buttons) as i64;
    }
    result as i32
}

/* -------------------- Part 2 (Branch & Bound on remaining vector) -------------------- */

fn part2(input: &str) -> i64 {
    todo!();
}
