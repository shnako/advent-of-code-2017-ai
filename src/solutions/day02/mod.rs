// Day 2: Corruption Checksum
// https://adventofcode.com/2017/day/2

/// Parse a row of whitespace-separated numbers
fn parse_row(row: &str) -> Vec<u32> {
    row.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// Solve part 1: Calculate checksum by summing differences between max and min of each row
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers = parse_row(line);
            if numbers.is_empty() {
                0
            } else {
                let max = *numbers.iter().max().unwrap();
                let min = *numbers.iter().min().unwrap();
                max - min
            }
        })
        .sum()
}

/// Solve part 2: Find the two numbers in each row where one evenly divides the other
/// and sum the division results
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers = parse_row(line);
            
            // Find the pair where one evenly divides the other
            for i in 0..numbers.len() {
                for j in i + 1..numbers.len() {
                    let (a, b) = (numbers[i], numbers[j]);
                    
                    // Skip if either number is 0 to avoid division/modulo by zero
                    if a == 0 || b == 0 {
                        continue;
                    }
                    
                    // Check if a divides b evenly
                    if b % a == 0 {
                        return b / a;
                    }
                    
                    // Check if b divides a evenly
                    if a % b == 0 {
                        return a / b;
                    }
                }
            }
            
            // Should not reach here based on problem statement
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        let example = "5 1 9 5
7 5 3
2 4 6 8";
        
        // Test individual rows
        assert_eq!(parse_row("5 1 9 5"), vec![5, 1, 9, 5]);
        
        // The first row's difference is 9 - 1 = 8
        // The second row's difference is 7 - 3 = 4  
        // The third row's difference is 8 - 2 = 6
        // Total checksum is 8 + 4 + 6 = 18
        assert_eq!(solve_part1(example), 18);
    }
    
    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day02/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 51139);
    }
    
    #[test]
    fn test_part2_examples() {
        let example = "5 9 2 8
9 4 7 3
3 8 6 5";
        
        // First row: 8 / 2 = 4
        // Second row: 9 / 3 = 3
        // Third row: 6 / 3 = 2
        // Total: 4 + 3 + 2 = 9
        assert_eq!(solve_part2(example), 9);
    }
    
    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day02/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 272);
    }
}