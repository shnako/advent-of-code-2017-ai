// Day 17: Spinlock
// https://adventofcode.com/2017/day/17

use std::collections::VecDeque;

pub fn solve_part1(input: &str) -> String {
    let steps: usize = input.trim().parse().unwrap();

    // Use VecDeque for efficient insertion
    let mut buffer = VecDeque::new();
    buffer.push_back(0);
    let mut position = 0;

    for value in 1..=2017 {
        // Step forward
        position = (position + steps) % buffer.len() + 1;

        // Insert value at position
        buffer.insert(position, value);
    }

    // Find the value after 2017
    let index_2017 = buffer.iter().position(|&x| x == 2017).unwrap();
    let next_index = (index_2017 + 1) % buffer.len();

    buffer[next_index].to_string()
}

pub fn solve_part2(input: &str) -> String {
    let steps: usize = input.trim().parse().unwrap();

    // We don't need to build the full buffer.
    // 0 is always at position 0, so we just track what's at position 1
    let mut position = 0;
    let mut value_after_zero = 0;

    for value in 1..=50_000_000 {
        // Calculate where we'd be after stepping
        position = (position + steps) % value + 1;

        // If we insert at position 1, that's after 0
        if position == 1 {
            value_after_zero = value;
        }
    }

    value_after_zero.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1("3"), "638");
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day17/input.txt").unwrap();
        assert_eq!(solve_part1(&input), "417");
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day17/input.txt").unwrap();
        assert_eq!(solve_part2(&input), "34334221");
    }
}
