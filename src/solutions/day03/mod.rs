// Day 3: Spiral Memory
// https://adventofcode.com/2017/day/3

/// Solve part 1: Calculate Manhattan distance from a given square to the center
pub fn solve_part1(input: &str) -> i32 {
    let n = input.trim().parse::<i32>().expect("Invalid input");

    // Special case: center square
    if n == 1 {
        return 0;
    }

    // Find which "ring" the number is in
    // Ring 0: just 1
    // Ring 1: 2-9 (8 numbers)
    // Ring 2: 10-25 (16 numbers)
    // Ring k: has 8k numbers

    // The last number in ring k is (2k+1)^2
    // Find the ring by finding the smallest odd number whose square is >= n
    let mut ring: i32 = 0;
    let mut max_in_ring: i64 = 1;

    while max_in_ring < n as i64 {
        ring += 1;
        let side = 2 * ring + 1;
        max_in_ring = (side as i64) * (side as i64);
    }

    // Now we know n is in ring `ring`
    // Each ring has 4 sides of length 2*ring
    // The minimum distance from any position in ring k to center is k
    // The maximum distance is 2*k (at corners)

    // Find position within the ring
    let side_length = 2 * ring;
    let ring_start = ((2 * ring - 1) * (2 * ring - 1)) + 1;
    let position_in_ring = n - ring_start;

    // Each side has side_length numbers
    // The middle of each side is at minimum distance (ring)
    // The corners are at maximum distance (2 * ring)

    // Find which side we're on and position within that side
    let position_in_side = position_in_ring % side_length;

    // Distance from middle of the side
    let distance_from_middle = (position_in_side - (side_length / 2 - 1)).abs();

    // Manhattan distance is ring distance + distance from middle of side
    ring + distance_from_middle
}

/// Solve part 2: Find the first value in the spiral that is larger than the input
/// where each value is the sum of all adjacent values (including diagonals)
pub fn solve_part2(input: &str) -> i32 {
    let target = input.trim().parse::<i64>().expect("Invalid input");

    use std::collections::HashMap;

    // Store values at each position
    let mut grid: HashMap<(i32, i32), i64> = HashMap::new();

    // Start at center with value 1
    grid.insert((0, 0), 1);

    // Direction vectors: right, up, left, down
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir_idx = 0;

    let mut x = 0;
    let mut y = 0;
    let mut steps_in_direction = 1;
    let mut steps_taken = 0;
    let mut times_turned = 0;

    loop {
        // Move in current direction
        let (dx, dy) = directions[dir_idx];
        x += dx;
        y += dy;

        // Calculate value as sum of all adjacent cells (including diagonals)
        let mut value: i64 = 0;
        for adj_x in -1..=1 {
            for adj_y in -1..=1 {
                if adj_x == 0 && adj_y == 0 {
                    continue;
                }
                if let Some(&adj_val) = grid.get(&(x + adj_x, y + adj_y)) {
                    value += adj_val;
                }
            }
        }

        // If this is the first value larger than target, return it
        if value > target {
            return value as i32;
        }

        grid.insert((x, y), value);

        steps_taken += 1;

        // Check if we need to turn
        if steps_taken == steps_in_direction {
            steps_taken = 0;
            dir_idx = (dir_idx + 1) % 4;
            times_turned += 1;

            // Increase steps after every 2 turns (completing a "ring")
            if times_turned % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part1("1"), 0);
        assert_eq!(solve_part1("12"), 3);
        assert_eq!(solve_part1("23"), 2);
        assert_eq!(solve_part1("1024"), 31);
    }

    #[test]
    fn test_part1_input() {
        let input = include_str!("input.txt");
        let result = solve_part1(input);
        assert_eq!(result, 419);
    }

    #[test]
    fn test_part2_examples() {
        // Test that the spiral values are calculated correctly
        // Values larger than 1: 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806...
        assert_eq!(solve_part2("1"), 2); // First value > 1 is 2
        assert_eq!(solve_part2("2"), 4); // First value > 2 is 4
        assert_eq!(solve_part2("10"), 11); // First value > 10 is 11
        assert_eq!(solve_part2("23"), 25); // First value > 23 is 25
        assert_eq!(solve_part2("747"), 806); // First value > 747 is 806
    }

    #[test]
    fn test_part2_input() {
        let input = include_str!("input.txt");
        let result = solve_part2(input);
        assert_eq!(result, 295229);
    }
}
