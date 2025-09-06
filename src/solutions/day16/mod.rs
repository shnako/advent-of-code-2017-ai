// Day 16: Permutation Promenade
// https://adventofcode.com/2017/day/16

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum DanceMove {
    Spin(usize),           // sX: move X programs from end to front
    Exchange(usize, usize), // xA/B: swap programs at positions A and B
    Partner(char, char),    // pA/B: swap programs named A and B
}

/// Parse dance moves from input
fn parse_moves(input: &str) -> Vec<DanceMove> {
    input
        .trim()
        .split(',')
        .map(|move_str| {
            let chars: Vec<char> = move_str.chars().collect();
            match chars[0] {
                's' => {
                    let size: usize = move_str[1..].parse().unwrap();
                    DanceMove::Spin(size)
                }
                'x' => {
                    let parts: Vec<&str> = move_str[1..].split('/').collect();
                    let pos_a: usize = parts[0].parse().unwrap();
                    let pos_b: usize = parts[1].parse().unwrap();
                    DanceMove::Exchange(pos_a, pos_b)
                }
                'p' => {
                    let prog_a = chars[1];
                    let prog_b = chars[3];
                    DanceMove::Partner(prog_a, prog_b)
                }
                _ => panic!("Unknown move type: {}", chars[0]),
            }
        })
        .collect()
}

/// Apply a single dance move to the programs
fn apply_move(programs: &mut Vec<char>, dance_move: &DanceMove) {
    match dance_move {
        DanceMove::Spin(size) => {
            let len = programs.len();
            let split_pos = len - size;
            let mut new_programs = Vec::with_capacity(len);
            new_programs.extend_from_slice(&programs[split_pos..]);
            new_programs.extend_from_slice(&programs[..split_pos]);
            *programs = new_programs;
        }
        DanceMove::Exchange(a, b) => {
            programs.swap(*a, *b);
        }
        DanceMove::Partner(a, b) => {
            let pos_a = programs.iter().position(|&c| c == *a).unwrap();
            let pos_b = programs.iter().position(|&c| c == *b).unwrap();
            programs.swap(pos_a, pos_b);
        }
    }
}

/// Perform one complete dance
fn dance(programs: &mut Vec<char>, moves: &[DanceMove]) {
    for dance_move in moves {
        apply_move(programs, dance_move);
    }
}

/// Solve part 1: Find the order of programs after one dance
pub fn solve_part1(input: &str) -> String {
    let moves = parse_moves(input);
    let mut programs: Vec<char> = ('a'..='p').collect();
    
    dance(&mut programs, &moves);
    
    programs.iter().collect()
}

/// Solve part 2: Find the order after one billion dances
pub fn solve_part2(input: &str) -> String {
    let moves = parse_moves(input);
    let mut programs: Vec<char> = ('a'..='p').collect();
    
    // Detect cycle
    let mut seen: HashMap<Vec<char>, usize> = HashMap::new();
    let target = 1_000_000_000;
    
    for i in 0..target {
        if let Some(&cycle_start) = seen.get(&programs) {
            // Found a cycle!
            let cycle_length = i - cycle_start;
            let remaining = (target - i) % cycle_length;
            
            // Fast forward to the final position
            for _ in 0..remaining {
                dance(&mut programs, &moves);
            }
            return programs.iter().collect();
        }
        
        seen.insert(programs.clone(), i);
        dance(&mut programs, &moves);
    }
    
    programs.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        // Test with the smaller example from the puzzle
        let moves = vec![
            DanceMove::Spin(1),
            DanceMove::Exchange(3, 4),
            DanceMove::Partner('e', 'b'),
        ];
        
        let mut programs: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        
        // Apply each move
        for dance_move in &moves {
            apply_move(&mut programs, dance_move);
        }
        
        assert_eq!(programs.iter().collect::<String>(), "baedc");
    }
    
    #[test]
    fn test_parsing() {
        let input = "s1,x3/4,pe/b";
        let moves = parse_moves(input);
        
        assert_eq!(moves.len(), 3);
        matches!(moves[0], DanceMove::Spin(1));
        matches!(moves[1], DanceMove::Exchange(3, 4));
        matches!(moves[2], DanceMove::Partner('e', 'b'));
    }
    
    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day16/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, "cgpfhdnambekjiol");
    }
    
    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day16/input.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, "gjmiofcnaehpdlbk");
    }
}