// Day 8: I Heard You Like Registers
// https://adventofcode.com/2017/day/8

use std::collections::HashMap;

/// Parse an instruction from a line
fn parse_instruction(line: &str) -> Option<(String, i32, String, String, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 7 {
        return None;
    }

    let target_reg = parts[0].to_string();
    let operation = parts[1];
    let amount: i32 = parts[2].parse().ok()?;

    // Validate the operation and calculate the actual amount
    let actual_amount = match operation {
        "inc" => amount,
        "dec" => -amount,
        _ => return None, // Reject unknown operations
    };

    // Validate that parts[3] is exactly "if"
    if parts[3] != "if" {
        return None;
    }

    let condition_reg = parts[4].to_string();
    let condition_op = parts[5].to_string();
    let condition_val: i32 = parts[6].parse().ok()?;

    Some((
        target_reg,
        actual_amount,
        condition_reg,
        condition_op,
        condition_val,
    ))
}

/// Check if a condition is met
fn check_condition(reg_value: i32, op: &str, target: i32) -> bool {
    match op {
        ">" => reg_value > target,
        "<" => reg_value < target,
        ">=" => reg_value >= target,
        "<=" => reg_value <= target,
        "==" => reg_value == target,
        "!=" => reg_value != target,
        _ => false,
    }
}

/// Solve part 1: Find the largest value in any register after completing all instructions
pub fn solve_part1(input: &str) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((target_reg, amount, condition_reg, condition_op, condition_val)) =
            parse_instruction(line)
        {
            // Get the condition register value (default to 0 if not yet initialized)
            let condition_reg_value = *registers.get(&condition_reg).unwrap_or(&0);

            // Check if the condition is met
            if check_condition(condition_reg_value, &condition_op, condition_val) {
                // Modify the target register
                let entry = registers.entry(target_reg).or_insert(0);
                *entry += amount;
            }
        }
    }

    // Find the maximum value in any register
    *registers.values().max().unwrap_or(&0)
}

/// Solve part 2: Find the highest value held in any register during the entire process
pub fn solve_part2(input: &str) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_ever = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((target_reg, amount, condition_reg, condition_op, condition_val)) =
            parse_instruction(line)
        {
            // Get the condition register value (default to 0 if not yet initialized)
            let condition_reg_value = *registers.get(&condition_reg).unwrap_or(&0);

            // Check if the condition is met
            if check_condition(condition_reg_value, &condition_op, condition_val) {
                // Modify the target register
                let entry = registers.entry(target_reg).or_insert(0);
                *entry += amount;

                // Track the highest value ever seen
                if *entry > highest_ever {
                    highest_ever = *entry;
                }
            }
        }
    }

    highest_ever
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        let example = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(solve_part1(example), 1);
    }

    #[test]
    fn test_part1_input() {
        let input =
            input::read_input("src/solutions/day08/input.txt").expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, 4902);
    }

    #[test]
    fn test_part2_examples() {
        let example = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(solve_part2(example), 10);
    }

    #[test]
    fn test_part2_input() {
        let input =
            input::read_input("src/solutions/day08/input.txt").expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 7037);
    }
}
