// Day 13: Packet Scanners
// https://adventofcode.com/2017/day/13

use std::collections::HashMap;

/// Parse the input to get the firewall layers
fn parse_input(input: &str) -> HashMap<usize, usize> {
    let mut layers = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse line like "0: 3"
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let depth: usize = parts[0].parse().unwrap();
        let range: usize = parts[1].parse().unwrap();
        layers.insert(depth, range);
    }

    layers
}

/// Calculate if a scanner is at position 0 at a given time
/// A scanner with range R moves in a pattern: 0, 1, 2, ..., R-1, R-2, ..., 1, 0, ...
/// This completes a full cycle in 2*(R-1) steps
fn scanner_at_top(range: usize, time: usize) -> bool {
    if range <= 1 {
        return true; // Always at top if range is 1
    }
    time % (2 * (range - 1)) == 0
}

/// Solve part 1: Calculate the severity of the trip if we leave immediately
/// We enter layer at depth D at time D picoseconds
/// We get caught if the scanner is at position 0 when we enter
/// Severity = sum of (depth * range) for layers where we're caught
pub fn solve_part1(input: &str) -> usize {
    let layers = parse_input(input);
    let mut severity = 0;

    for (&depth, &range) in &layers {
        // We reach this layer at time = depth
        if scanner_at_top(range, depth) {
            severity += depth * range;
        }
    }

    severity
}

/// Check if we get caught with a given delay
/// Returns true if we get caught at any layer
fn is_caught(layers: &HashMap<usize, usize>, delay: usize) -> bool {
    for (&depth, &range) in layers {
        // We reach this layer at time = delay + depth
        if scanner_at_top(range, delay + depth) {
            return true;
        }
    }
    false
}

/// Solve part 2: Find the minimum delay before starting
/// We need to find the minimum delay such that we don't get caught at any layer
pub fn solve_part2(input: &str) -> usize {
    let layers = parse_input(input);

    // Start from delay 0 and increment until we find a safe path
    let mut delay = 0;
    while is_caught(&layers, delay) {
        delay += 1;
    }

    delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(solve_part1(input), 24);
    }

    #[test]
    fn test_scanner_at_top() {
        // Range 3: positions 0, 1, 2, 1, 0, 1, 2, 1, 0...
        // Cycle length = 2*(3-1) = 4
        assert!(scanner_at_top(3, 0));
        assert!(!scanner_at_top(3, 1));
        assert!(!scanner_at_top(3, 2));
        assert!(!scanner_at_top(3, 3));
        assert!(scanner_at_top(3, 4));

        // Range 2: positions 0, 1, 0, 1, 0...
        // Cycle length = 2*(2-1) = 2
        assert!(scanner_at_top(2, 0));
        assert!(!scanner_at_top(2, 1));
        assert!(scanner_at_top(2, 2));

        // Range 4: positions 0, 1, 2, 3, 2, 1, 0...
        // Cycle length = 2*(4-1) = 6
        assert!(scanner_at_top(4, 0));
        assert!(!scanner_at_top(4, 1));
        assert!(!scanner_at_top(4, 2));
        assert!(!scanner_at_top(4, 3));
        assert!(!scanner_at_top(4, 4));
        assert!(!scanner_at_top(4, 5));
        assert!(scanner_at_top(4, 6));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day13/input.txt").unwrap();
        assert_eq!(solve_part1(&input), 1640);
    }

    #[test]
    fn test_part2_example() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(solve_part2(input), 10);
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day13/input.txt").unwrap();
        assert_eq!(solve_part2(&input), 3960702);
    }
}
