use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) {
    println!("=== Day 7 ===");
    
    let grid = parse_grid(input);
    let start = find_start(&grid);
    let first = Cord::new(start.x, start.y + 1);
    
    println!("Part 1: {}", part1(&grid, first));
    println!("Part 2: {}", part2(&grid, first));
}

fn part1(grid: &[Vec<char>], start: Cord) -> i32 {
    let mut queue: VecDeque<Cord> = VecDeque::new();
    queue.push_back(start);
    count_beam_splitting(&mut queue, grid)
}

fn part2(grid: &[Vec<char>], start: Cord) -> u128 {
    count_timelines_dp(grid, start)
}

// Helper functions
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> Cord {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == 'S' {
                return Cord::new(x as i32, y as i32);
            }
        }
    }
    panic!("Hittade ingen 'S' i gridet");
}

fn count_beam_splitting(queue: &mut VecDeque<Cord>, grid: &[Vec<char>]) -> i32 {
    let mut splits = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    while let Some(mut cord) = queue.pop_front() {
        // om vi redan har processat denna startpunkt, hoppa
        if seen.contains(&(cord.x, cord.y)) {
            continue;
        }

        loop {
            // markera att vi nu processar denna position
            if !seen.insert((cord.x, cord.y)) {
                break; // framtiden från denna punkt är redan simulerad
            }

            match cord.move_down(grid) {
                State::Free => {
                    // move_down har redan flyttat ner self.y
                    continue;
                }
                State::Split(left, right) => {
                    // enqueue:a right om den inte redan är sedd
                    if !seen.contains(&(right.x, right.y)) {
                        queue.push_back(right);
                    }

                    cord = left;
                    splits += 1;
                }
                State::End => break,
            }
        }
    }

    splits
}

fn count_timelines_dp(grid: &[Vec<char>], start: Cord) -> u128 {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    // ways för aktuell rad: x -> antal tidslinjer som "är vid" (x, y)
    let mut ways: HashMap<i32, u128> = HashMap::new();
    ways.insert(start.x, 1);

    let mut y = start.y;
    let mut done: u128 = 0;

    while y < h {
        let mut next: HashMap<i32, u128> = HashMap::new();

        for (&x, &cnt) in ways.iter() {
            // om man hamnar utanför
            if x < 0 || x >= w {
                done += cnt;
                continue;
            }

            let cell = grid[y as usize][x as usize];
            match cell {
                '.' => {
                    // fortsätt rakt ner
                    *next.entry(x).or_insert(0) += cnt;
                }
                '^' => {
                    // split: vänster/höger på nästa rad
                    *next.entry(x - 1).or_insert(0) += cnt;
                    *next.entry(x + 1).or_insert(0) += cnt;
                }
                _ => {
                    done += cnt;
                }
            }
        }

        ways = next;
        y += 1;
    }

    // allt som fortfarande är "på väg" efter sista raden lämnar grid
    done + ways.values().copied().sum::<u128>()
}

// Types

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Split(Cord, Cord),
    Free,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cord {
    x: i32,
    y: i32,
}

impl Cord {
    fn new(x: i32, y: i32) -> Self {
        Cord { x, y }
    }

    fn move_down(&mut self, grid: &[Vec<char>]) -> State {
        if self.x < 0 || self.y < 0 {
            return State::End;
        }

        let y = self.y as usize;
        let x = self.x as usize;

        if y >= grid.len() || x >= grid[y].len() {
            return State::End;
        }
        let next_step_c = grid[y][x];

        match next_step_c {
            '^' => {
                let left = Cord::new(self.x - 1, self.y);
                let right = Cord::new(self.x + 1, self.y);
                State::Split(left, right)
            }
            '.' => {
                self.y += 1;
                State::Free
            }
            _ => State::End,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...S...
.......
..^....
.......
.......";

    #[test]
    fn test_parse_and_find_start() {
        let grid = parse_grid(EXAMPLE);
        let start = find_start(&grid);
        assert_eq!(start.x, 3);
        assert_eq!(start.y, 0);
    }
}
