// Day 22: Sporifica Virus

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn move_forward(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

fn parse_input(input: &str) -> HashMap<(i32, i32), NodeState> {
    let mut grid = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    // Center the grid at (0, 0)
    let offset_x = width / 2;
    let offset_y = height / 2;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as i32 - offset_x, y as i32 - offset_y);
            if ch == '#' {
                grid.insert(pos, NodeState::Infected);
            }
        }
    }

    grid
}

pub fn solve_part1(input: &str) -> i32 {
    let mut grid = parse_input(input);
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let mut infection_count = 0;

    for _ in 0..10000 {
        // Get current node state (default to Clean if not in grid)
        let current_state = *grid.get(&position).unwrap_or(&NodeState::Clean);

        // Turn based on current node state
        direction = match current_state {
            NodeState::Infected => direction.turn_right(),
            NodeState::Clean => direction.turn_left(),
            _ => direction, // For part 1, we only have Clean and Infected
        };

        // Toggle the current node state
        match current_state {
            NodeState::Clean => {
                grid.insert(position, NodeState::Infected);
                infection_count += 1;
            }
            NodeState::Infected => {
                grid.remove(&position); // Remove from map to save memory (clean nodes)
            }
            _ => {} // For part 1, we only have Clean and Infected
        }

        // Move forward
        position = direction.move_forward(position);
    }

    infection_count
}

pub fn solve_part2(input: &str) -> i32 {
    let mut grid = parse_input(input);
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let mut infection_count = 0;

    for _ in 0..10000000 {
        // Get current node state (default to Clean if not in grid)
        let current_state = *grid.get(&position).unwrap_or(&NodeState::Clean);

        // Turn based on current node state
        direction = match current_state {
            NodeState::Clean => direction.turn_left(),
            NodeState::Weakened => direction, // No turn
            NodeState::Infected => direction.turn_right(),
            NodeState::Flagged => direction.reverse(),
        };

        // Update the current node state
        match current_state {
            NodeState::Clean => {
                grid.insert(position, NodeState::Weakened);
            }
            NodeState::Weakened => {
                grid.insert(position, NodeState::Infected);
                infection_count += 1;
            }
            NodeState::Infected => {
                grid.insert(position, NodeState::Flagged);
            }
            NodeState::Flagged => {
                grid.remove(&position); // Clean node
            }
        }

        // Move forward
        position = direction.move_forward(position);
    }

    infection_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let input = "..#\n#..\n...";

        // According to the puzzle, the example should give 5587 infections after 10000 bursts
        assert_eq!(solve_part1(input), 5587);
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day22/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 5447);
    }

    #[test]
    fn test_part2_examples() {
        let input = "..#\n#..\n...";

        // According to the puzzle, the example should give 2511944 infections after 10000000 bursts
        assert_eq!(solve_part2(input), 2511944);
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day22/input.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 2511705);
    }
}
