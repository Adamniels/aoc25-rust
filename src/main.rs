mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <day> [--example]");
        println!("Example: cargo run 1");
        println!("         cargo run 1 --example");
        return;
    }

    let day: u8 = args[1].parse().expect("Invalid day number");
    let use_example = args.contains(&"--example".to_string());

    let input = read_input(day, use_example);

    match day {
        1 => days::day01::solve(&input),
        // 2 => days::day02::solve(&input),
        // 3 => days::day03::solve(&input),
        // 4 => days::day04::solve(&input),
        // 5 => days::day05::solve(&input),
        // 6 => days::day06::solve(&input),
        7 => days::day07::solve(&input),
        8 => days::day08::solve(&input),
        _ => println!("Day {} not implemented yet", day),
    }
}

fn read_input(day: u8, use_example: bool) -> String {
    let filename = if use_example {
        format!("inputs/day{:02}_example.txt", day)
    } else {
        format!("inputs/day{:02}.txt", day)
    };

    std::fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename))
}
