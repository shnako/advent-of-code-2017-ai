// Day 12: Digital Plumber
// https://adventofcode.com/2017/day/12

use std::collections::{HashMap, HashSet};
use crate::utils::graph::{Graph, find_reachable};

/// Parse the input to build a graph of program connections
fn parse_input(input: &str) -> Graph<u32> {
    let mut graph = HashMap::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse line like "0 <-> 46, 1376"
        let parts: Vec<&str> = line.split(" <-> ").collect();
        if parts.len() != 2 {
            continue;
        }
        
        let program_id: u32 = parts[0].parse().unwrap();
        let connections: Vec<u32> = parts[1]
            .split(", ")
            .filter_map(|s| s.parse().ok())
            .collect();
        
        graph.insert(program_id, connections);
    }
    
    graph
}

/// Solve part 1: Find how many programs are in the group that contains program ID 0
/// Approach: Parse the input to build a graph, then use BFS to find all programs
/// reachable from program 0.
pub fn solve_part1(input: &str) -> usize {
    let graph = parse_input(input);
    let reachable = find_reachable(&graph, &0);
    reachable.len()
}

/// Solve part 2: Count the total number of groups
/// Approach: Find all connected components in the graph. Start with any unvisited node,
/// find all reachable nodes from it (which forms a group), mark them as visited,
/// and repeat until all nodes are visited.
pub fn solve_part2(input: &str) -> usize {
    let graph = parse_input(input);
    
    // Get all program IDs
    let mut unvisited: HashSet<u32> = graph.keys().cloned().collect();
    let mut group_count = 0;
    
    // Find all groups
    while !unvisited.is_empty() {
        // Pick any unvisited program
        let start_program = *unvisited.iter().next().unwrap();
        
        // Find all programs in this group
        let group = find_reachable(&graph, &start_program);
        
        // Remove all programs in this group from unvisited
        for program in group {
            unvisited.remove(&program);
        }
        
        group_count += 1;
    }
    
    group_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        let example = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(solve_part1(example), 6);
    }

    #[test]
    fn test_part1_input() {
        let input = input::read_input("src/solutions/day12/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 152);
    }

    #[test]
    fn test_part2_examples() {
        let example = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(solve_part2(example), 2);
    }

    #[test]
    fn test_part2_input() {
        let input = input::read_input("src/solutions/day12/input.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 186);
    }
}