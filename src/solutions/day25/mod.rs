// Day 25: The Halting Problem
// https://adventofcode.com/2017/day/25
//
// Part 1: Simulate a Turing machine following the given blueprint and count 1s on tape
// Part 2: No Part 2 for Day 25 - this is the final day of Advent of Code

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct StateRule {
    write_value: i32,
    direction: Direction,
    next_state: char,
}

#[derive(Debug, Clone)]
struct State {
    rules: HashMap<i32, StateRule>,
}

struct TuringMachine {
    tape: HashMap<i32, i32>,
    cursor: i32,
    current_state: char,
    states: HashMap<char, State>,
    steps_to_run: usize,
}

impl TuringMachine {
    fn parse(input: &str) -> TuringMachine {
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;
        
        // Parse initial state
        let initial_state = lines[i]
            .trim_start_matches("Begin in state ")
            .trim_end_matches(".")
            .chars()
            .next()
            .unwrap();
        i += 1;
        
        // Parse steps
        let steps_to_run = lines[i]
            .trim_start_matches("Perform a diagnostic checksum after ")
            .trim_end_matches(" steps.")
            .parse()
            .unwrap();
        i += 2; // Skip empty line
        
        let mut states = HashMap::new();
        
        while i < lines.len() {
            if lines[i].starts_with("In state ") {
                let state_char = lines[i]
                    .trim_start_matches("In state ")
                    .trim_end_matches(":")
                    .chars()
                    .next()
                    .unwrap();
                i += 1;
                
                let mut rules = HashMap::new();
                
                // Parse rule for value 0
                if i < lines.len() && lines[i].contains("If the current value is 0:") {
                    i += 1;
                    let write_value = if lines[i].contains("Write the value 1") { 1 } else { 0 };
                    i += 1;
                    let direction = if lines[i].contains("to the right") {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                    i += 1;
                    let next_state = lines[i]
                        .trim_start_matches("    - Continue with state ")
                        .trim_end_matches(".")
                        .chars()
                        .next()
                        .unwrap();
                    i += 1;
                    
                    rules.insert(0, StateRule {
                        write_value,
                        direction,
                        next_state,
                    });
                }
                
                // Parse rule for value 1
                if i < lines.len() && lines[i].contains("If the current value is 1:") {
                    i += 1;
                    let write_value = if lines[i].contains("Write the value 1") { 1 } else { 0 };
                    i += 1;
                    let direction = if lines[i].contains("to the right") {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                    i += 1;
                    let next_state = lines[i]
                        .trim_start_matches("    - Continue with state ")
                        .trim_end_matches(".")
                        .chars()
                        .next()
                        .unwrap();
                    i += 1;
                    
                    rules.insert(1, StateRule {
                        write_value,
                        direction,
                        next_state,
                    });
                }
                
                states.insert(state_char, State { rules });
            }
            i += 1;
        }
        
        TuringMachine {
            tape: HashMap::new(),
            cursor: 0,
            current_state: initial_state,
            states,
            steps_to_run,
        }
    }
    
    fn run(&mut self) -> i32 {
        for _ in 0..self.steps_to_run {
            let current_value = *self.tape.get(&self.cursor).unwrap_or(&0);
            let state = &self.states[&self.current_state];
            let rule = &state.rules[&current_value];
            
            // Write value
            if rule.write_value == 0 {
                self.tape.remove(&self.cursor);
            } else {
                self.tape.insert(self.cursor, rule.write_value);
            }
            
            // Move cursor
            match rule.direction {
                Direction::Left => self.cursor -= 1,
                Direction::Right => self.cursor += 1,
            }
            
            // Change state
            self.current_state = rule.next_state;
        }
        
        // Count 1s on the tape
        self.tape.values().filter(|&&v| v == 1).count() as i32
    }
}

pub fn solve_part1(input: &str) -> i32 {
    let mut machine = TuringMachine::parse(input);
    machine.run()
}

pub fn solve_part2(_input: &str) -> i32 {
    // Day 25 typically doesn't have a Part 2 in Advent of Code
    // It's the final day and usually just requires solving Part 1
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
        
        assert_eq!(solve_part1(input), 3);
    }

    #[test] 
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day25/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 2725);
    }
}