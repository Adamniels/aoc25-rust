pub fn solve(input: &str) {
    println!("=== Day 1 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                return 0;
            }
            let first = digits.first().unwrap().to_digit(10).unwrap() as i32;
            let last = digits.last().unwrap().to_digit(10).unwrap() as i32;
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    part1(input) // Placeholder - replace with actual solution
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 142);
    }

    #[test]
    fn test_part2() {
        // Update with expected result for part 2
        assert_eq!(part2(EXAMPLE), 142);
    }
}
