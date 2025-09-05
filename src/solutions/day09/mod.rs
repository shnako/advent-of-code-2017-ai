// Day 9: Stream Processing
// https://adventofcode.com/2017/day/9

/// Process the stream and return the total score of all groups
/// Approach: Parse character by character, tracking depth and garbage state.
/// Groups contribute their depth level to the total score when closed.
pub fn solve_part1(input: &str) -> i32 {
    let chars: Vec<char> = input.trim().chars().collect();
    let mut i = 0;
    let mut depth = 0;
    let mut total_score = 0;
    let mut in_garbage = false;
    
    while i < chars.len() {
        if in_garbage {
            if chars[i] == '!' {
                // Skip the next character
                i += 2;
            } else if chars[i] == '>' {
                // End garbage
                in_garbage = false;
                i += 1;
            } else {
                // Skip character in garbage
                i += 1;
            }
        } else {
            match chars[i] {
                '{' => {
                    // Start a new group
                    depth += 1;
                    i += 1;
                }
                '}' => {
                    // Close a group and add its score
                    total_score += depth;
                    depth -= 1;
                    i += 1;
                }
                '<' => {
                    // Start garbage
                    in_garbage = true;
                    i += 1;
                }
                ',' => {
                    // Skip commas
                    i += 1;
                }
                _ => {
                    // Shouldn't happen in valid input
                    i += 1;
                }
            }
        }
    }
    
    total_score
}

/// Count all non-canceled characters within the garbage
/// Approach: Similar parsing but focus on counting characters inside garbage blocks.
/// Exclude opening/closing brackets and canceled characters after '!'.
pub fn solve_part2(input: &str) -> i32 {
    let chars: Vec<char> = input.trim().chars().collect();
    let mut i = 0;
    let mut in_garbage = false;
    let mut garbage_count = 0;
    
    while i < chars.len() {
        if in_garbage {
            if chars[i] == '!' {
                // Skip the next character (both the ! and the next char don't count)
                i += 2;
            } else if chars[i] == '>' {
                // End garbage (the > doesn't count)
                in_garbage = false;
                i += 1;
            } else {
                // Count this character as garbage
                garbage_count += 1;
                i += 1;
            }
        } else {
            match chars[i] {
                '<' => {
                    // Start garbage (the < doesn't count)
                    in_garbage = true;
                    i += 1;
                }
                _ => {
                    // Skip everything else outside garbage
                    i += 1;
                }
            }
        }
    }
    
    garbage_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        // Test cases from the puzzle description
        assert_eq!(solve_part1("{}"), 1);
        assert_eq!(solve_part1("{{{}}}"), 6);
        assert_eq!(solve_part1("{{},{}}"), 5);
        assert_eq!(solve_part1("{{{},{},{{}}}}"), 16);
        assert_eq!(solve_part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(solve_part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(solve_part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(solve_part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day09/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 10820);
    }

    #[test]
    fn test_part2_examples() {
        // Test cases from the puzzle description
        assert_eq!(solve_part2("<>"), 0);
        assert_eq!(solve_part2("<random characters>"), 17);
        assert_eq!(solve_part2("<<<<>"), 3);
        assert_eq!(solve_part2("<{!>}>"), 2);
        assert_eq!(solve_part2("<!!>"), 0);
        assert_eq!(solve_part2("<!!!>>"), 0);
        assert_eq!(solve_part2("<{o\"i!a,<{i<a>"), 10);
    }

    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day09/input.txt")
            .expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 5547);
    }
}