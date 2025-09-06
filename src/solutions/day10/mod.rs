// Day 10: Knot Hash
// https://adventofcode.com/2017/day/10

/// Parse the input to get the sequence of lengths
fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// Solve part 1: Implement the Knot Hash algorithm and multiply the first two numbers
/// Approach: Parse comma-separated lengths, perform knot hash operations on a circular list of 0-255,
/// reversing subsequences while tracking position and skip size, then return product of first two elements.
pub fn solve_part1(input: &str) -> u32 {
    let lengths = parse_input(input);

    // Initialize the list with numbers from 0 to 255
    let mut list: Vec<u32> = (0..256).collect();
    let list_size = list.len();

    let mut current_position = 0;
    let mut skip_size = 0;

    for length in lengths {
        if length > list_size {
            // Invalid length, skip it
            continue;
        }

        // Reverse the order of `length` elements starting at current_position
        // Handle wrapping around the circular list
        let mut indices = Vec::new();
        for i in 0..length {
            indices.push((current_position + i) % list_size);
        }

        // Extract the values to reverse
        let mut values: Vec<u32> = indices.iter().map(|&i| list[i]).collect();
        values.reverse();

        // Put the reversed values back
        for (i, &idx) in indices.iter().enumerate() {
            list[idx] = values[i];
        }

        // Move current position forward by length + skip_size
        current_position = (current_position + length + skip_size) % list_size;

        // Increase skip size
        skip_size += 1;
    }

    // Multiply the first two numbers
    list[0] * list[1]
}

/// Solve part 2: Full Knot Hash with ASCII conversion, 64 rounds, dense hash, and hex output
/// Approach: Convert input to ASCII bytes plus standard suffix, run 64 rounds of knot hash,
/// create dense hash by XORing 16-element blocks, then format as 32-character hexadecimal string.
pub fn solve_part2(input: &str) -> String {
    crate::utils::hash::knot_hash(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        // For the example, we need to use a smaller list (0-4) instead of 0-255
        // Let's create a specialized test function for this
        fn solve_part1_example(input: &str, list_size: usize) -> u32 {
            let lengths = parse_input(input);

            // Initialize the list with numbers from 0 to list_size-1
            let mut list: Vec<u32> = (0..list_size as u32).collect();

            let mut current_position = 0;
            let mut skip_size = 0;

            for length in lengths {
                if length > list_size {
                    continue;
                }

                // Reverse the order of `length` elements starting at current_position
                let mut indices = Vec::new();
                for i in 0..length {
                    indices.push((current_position + i) % list_size);
                }

                let mut values: Vec<u32> = indices.iter().map(|&i| list[i]).collect();
                values.reverse();

                for (i, &idx) in indices.iter().enumerate() {
                    list[idx] = values[i];
                }

                current_position = (current_position + length + skip_size) % list_size;
                skip_size += 1;
            }

            // Multiply the first two numbers
            list[0] * list[1]
        }

        // Example from the problem: list of 5 elements (0-4), lengths: 3,4,1,5
        // Expected result: 3 * 4 = 12
        assert_eq!(solve_part1_example("3,4,1,5", 5), 12);
    }

    #[test]
    fn test_part1_input() {
        let input =
            input::read_input("src/solutions/day10/input.txt").expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 6909);
    }

    #[test]
    fn test_part2_examples() {
        // Test the examples from the problem description
        assert_eq!(solve_part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solve_part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solve_part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solve_part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn test_part2_input() {
        let input =
            input::read_input("src/solutions/day10/input.txt").expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, "9d5f4561367d379cfbf04f8c471c0095");
    }
}
