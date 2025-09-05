// Day 11: Hex Ed
// https://adventofcode.com/2017/day/11

/// Represents a position in hex grid using cube coordinates
/// In cube coordinates, x + y + z = 0 always holds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl HexPosition {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
    
    /// Move in a given direction
    fn move_direction(&mut self, direction: &str) {
        match direction {
            "n" => {
                self.y += 1;
                self.z -= 1;
            }
            "ne" => {
                self.x += 1;
                self.z -= 1;
            }
            "se" => {
                self.x += 1;
                self.y -= 1;
            }
            "s" => {
                self.y -= 1;
                self.z += 1;
            }
            "sw" => {
                self.x -= 1;
                self.z += 1;
            }
            "nw" => {
                self.x -= 1;
                self.y += 1;
            }
            _ => {}
        }
    }
    
    /// Calculate the distance from origin to this position
    /// In cube coordinates, the distance is (|x| + |y| + |z|) / 2
    /// But since x + y + z = 0, we can simplify to max(|x|, |y|, |z|)
    fn distance_from_origin(&self) -> i32 {
        self.x.abs().max(self.y.abs()).max(self.z.abs())
    }
}

/// Parse the input to get the sequence of moves
fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// Solve part 1: Find the fewest number of steps to reach the child process
pub fn solve_part1(input: &str) -> i32 {
    let moves = parse_input(input);
    let mut position = HexPosition::new();
    
    // Follow all the moves
    for direction in moves {
        position.move_direction(&direction);
    }
    
    // Return the distance from the origin
    position.distance_from_origin()
}

/// Solve part 2: Find the furthest distance ever reached from the starting position
pub fn solve_part2(input: &str) -> i32 {
    let moves = parse_input(input);
    let mut position = HexPosition::new();
    let mut max_distance = 0;
    
    // Follow all the moves and track the maximum distance
    for direction in moves {
        position.move_direction(&direction);
        max_distance = max_distance.max(position.distance_from_origin());
    }
    
    max_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        // ne,ne,ne is 3 steps away
        assert_eq!(solve_part1("ne,ne,ne"), 3);
        
        // ne,ne,sw,sw is 0 steps away (back where you started)
        assert_eq!(solve_part1("ne,ne,sw,sw"), 0);
        
        // ne,ne,s,s is 2 steps away (se,se)
        assert_eq!(solve_part1("ne,ne,s,s"), 2);
        
        // se,sw,se,sw,sw is 3 steps away (s,s,sw)
        assert_eq!(solve_part1("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day11/input.txt").unwrap();
        assert_eq!(solve_part1(&input), 664);
    }

    #[test]
    fn test_part2_examples() {
        // The examples should track max distance during the journey
        // ne,ne,ne - moves 3 steps away, max is 3
        assert_eq!(solve_part2("ne,ne,ne"), 3);
        
        // ne,ne,sw,sw - goes out 2 steps then back to origin, max is 2
        assert_eq!(solve_part2("ne,ne,sw,sw"), 2);
        
        // ne,ne,s,s - goes out 2 steps then changes direction but stays at 2
        assert_eq!(solve_part2("ne,ne,s,s"), 2);
        
        // se,sw,se,sw,sw - varies in distance, need to track the max
        assert_eq!(solve_part2("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day11/input.txt").unwrap();
        assert_eq!(solve_part2(&input), 1447);
    }
}