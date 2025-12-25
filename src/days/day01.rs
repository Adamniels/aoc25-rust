pub fn solve(input: &str) {
    println!("=== Day 1 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
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
