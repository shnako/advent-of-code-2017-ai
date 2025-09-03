// Day 1: Inverse Captcha
// https://adventofcode.com/2017/day/1

use crate::utils::input;

/// Solve part 1: Sum of digits that match the next digit (circular)
pub fn solve_part1(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.is_empty() {
        return 0;
    }
    
    let mut sum = 0;
    let len = digits.len();
    
    for i in 0..len {
        let next_index = (i + 1) % len; // Circular: wrap around to the beginning
        if digits[i] == digits[next_index] {
            sum += digits[i];
        }
    }
    
    sum
}

/// Solve part 2: Sum of digits that match the digit halfway around the circular list
pub fn solve_part2(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.is_empty() {
        return 0;
    }
    
    let mut sum = 0;
    let len = digits.len();
    let step = len / 2; // Halfway around
    
    for i in 0..len {
        let compare_index = (i + step) % len; // Circular: wrap around
        if digits[i] == digits[compare_index] {
            sum += digits[i];
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        // Example 1: 1122 produces 3 (1 + 2)
        assert_eq!(solve_part1("1122"), 3);
        
        // Example 2: 1111 produces 4
        assert_eq!(solve_part1("1111"), 4);
        
        // Example 3: 1234 produces 0
        assert_eq!(solve_part1("1234"), 0);
        
        // Example 4: 91212129 produces 9
        assert_eq!(solve_part1("91212129"), 9);
    }
    
    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day01/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 1251);
    }
    
    #[test]
    fn test_part2_examples() {
        // Example 1: 1212 produces 6
        assert_eq!(solve_part2("1212"), 6);
        
        // Example 2: 1221 produces 0
        assert_eq!(solve_part2("1221"), 0);
        
        // Example 3: 123425 produces 4
        assert_eq!(solve_part2("123425"), 4);
        
        // Example 4: 123123 produces 12
        assert_eq!(solve_part2("123123"), 12);
        
        // Example 5: 12131415 produces 4
        assert_eq!(solve_part2("12131415"), 4);
    }
    
    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day01/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 1244);
    }
}