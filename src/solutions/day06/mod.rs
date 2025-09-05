// Day 6: Memory Reallocation
// https://adventofcode.com/2017/day/6

use std::collections::HashSet;

/// Parse the input into a vector of memory bank values
fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// Perform one redistribution cycle
fn redistribute(banks: &mut [usize]) {
    // Find the bank with the most blocks (ties won by lowest index)
    let (max_index, max_value) = banks
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| {
            // Compare values first, then use reverse index comparison for tie-breaking
            // (lower index wins, so we reverse the comparison)
            a.cmp(b).then_with(|| j.cmp(i))
        })
        .unwrap();

    let blocks_to_distribute = *max_value;
    banks[max_index] = 0;

    // Distribute the blocks starting from the next bank
    let mut current_index = (max_index + 1) % banks.len();
    for _ in 0..blocks_to_distribute {
        banks[current_index] += 1;
        current_index = (current_index + 1) % banks.len();
    }
}

/// Solve part 1: Count redistribution cycles until a repeated configuration is seen
pub fn solve_part1(input: &str) -> usize {
    let mut banks = parse_input(input);
    let mut seen_states = HashSet::new();
    let mut cycles = 0;

    // Add initial state
    seen_states.insert(banks.clone());

    loop {
        redistribute(&mut banks);
        cycles += 1;

        // Check if we've seen this state before
        if !seen_states.insert(banks.clone()) {
            return cycles;
        }
    }
}

/// Solve part 2: Size of the loop (cycles between repeated states)
pub fn solve_part2(input: &str) -> usize {
    let mut banks = parse_input(input);
    let mut seen_states = HashSet::new();

    // First, find the repeated state
    loop {
        if !seen_states.insert(banks.clone()) {
            // We found the repeated state, break out
            break;
        }
        redistribute(&mut banks);
    }

    // Now we have the repeated state in `banks`
    // Count how many cycles it takes to see this state again
    let target_state = banks.clone();
    let mut cycles = 0;

    loop {
        redistribute(&mut banks);
        cycles += 1;

        if banks == target_state {
            return cycles;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        // Example from the problem: 0 2 7 0
        // Should take 5 cycles to reach a repeated state
        assert_eq!(solve_part1("0 2 7 0"), 5);
    }

    #[test]
    fn test_part1_input() {
        let input =
            input::read_input("src/solutions/day06/input.txt").expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 14029);
    }

    #[test]
    fn test_part2_examples() {
        // Example from the problem: 0 2 7 0
        // The state "2 4 1 2" is seen again after 4 cycles
        assert_eq!(solve_part2("0 2 7 0"), 4);
    }

    #[test]
    fn test_part2_input() {
        let input =
            input::read_input("src/solutions/day06/input.txt").expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 2765);
    }
}
