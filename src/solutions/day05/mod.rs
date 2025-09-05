// Day 5: A Maze of Twisty Trampolines, All Alike
// https://adventofcode.com/2017/day/5

/// Parse the input into a vector of jump offsets
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().expect("Invalid number in input"))
        .collect()
}

/// Solve part 1: Count steps to exit the jump maze
pub fn solve_part1(input: &str) -> u32 {
    let mut jumps = parse_input(input);
    let mut position: i32 = 0;
    let mut steps = 0;
    
    while position >= 0 && (position as usize) < jumps.len() {
        let current_pos = position as usize;
        let offset = jumps[current_pos];
        
        // Jump to new position
        position += offset;
        
        // Increment the offset we just used
        jumps[current_pos] += 1;
        
        // Count the step
        steps += 1;
    }
    
    steps
}

/// Solve part 2: Count steps with modified increment rule
pub fn solve_part2(input: &str) -> u32 {
    let mut jumps = parse_input(input);
    let mut position: i32 = 0;
    let mut steps = 0;
    
    while position >= 0 && (position as usize) < jumps.len() {
        let current_pos = position as usize;
        let offset = jumps[current_pos];
        
        // Jump to new position
        position += offset;
        
        // Modified rule: if offset was 3 or more, decrease by 1; otherwise increase by 1
        if offset >= 3 {
            jumps[current_pos] -= 1;
        } else {
            jumps[current_pos] += 1;
        }
        
        // Count the step
        steps += 1;
    }
    
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let example = "0
3
0
1
-3";
        assert_eq!(solve_part1(example), 5);
    }

    #[test]
    fn test_part1_input() {
        let input = include_str!("input.txt");
        assert_eq!(solve_part1(input), 388611);
    }

    #[test]
    fn test_part2_examples() {
        let example = "0
3
0
1
-3";
        assert_eq!(solve_part2(example), 10);
    }

    #[test]
    fn test_part2_input() {
        let input = include_str!("input.txt");
        assert_eq!(solve_part2(input), 27763113);
    }
}