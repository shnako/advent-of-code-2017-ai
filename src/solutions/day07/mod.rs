// Day 7: Recursive Circus
// https://adventofcode.com/2017/day/7

use std::collections::{HashMap, HashSet};

/// Parse a line of input to extract program name, weight, and children
fn parse_line(line: &str) -> (String, i32, Vec<String>) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    
    // Parse name and weight
    let name_weight = parts[0];
    let space_idx = name_weight.find(' ').unwrap();
    let name = name_weight[..space_idx].to_string();
    let weight_str = &name_weight[space_idx + 1..];
    let weight: i32 = weight_str
        .trim_start_matches('(')
        .trim_end_matches(')')
        .parse()
        .unwrap();
    
    // Parse children if they exist
    let children = if parts.len() > 1 {
        parts[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };
    
    (name, weight, children)
}

/// Solve part 1: Find the name of the bottom program (root of the tree)
/// 
/// Approach: The root program is the only one that isn't held by any other program.
/// We track all programs and all held programs, then find the difference.
pub fn solve_part1(input: &str) -> String {
    let mut all_programs = HashSet::new();
    let mut held_programs = HashSet::new();
    
    // Parse all lines to find which programs are held by others
    for line in input.trim().lines() {
        let (name, _weight, children) = parse_line(line);
        all_programs.insert(name);
        
        // Add all children to the set of held programs
        for child in children {
            held_programs.insert(child);
        }
    }
    
    // The bottom program is the one that isn't held by any other
    for program in all_programs {
        if !held_programs.contains(&program) {
            return program;
        }
    }
    
    String::new() // This shouldn't happen with valid input
}

/// Build a tree structure from the input
fn build_tree(input: &str) -> (HashMap<String, i32>, HashMap<String, Vec<String>>, String) {
    let mut weights = HashMap::new();
    let mut children_map = HashMap::new();
    let mut all_programs = HashSet::new();
    let mut held_programs = HashSet::new();
    
    // Parse all lines
    for line in input.trim().lines() {
        let (name, weight, children) = parse_line(line);
        weights.insert(name.clone(), weight);
        
        if !children.is_empty() {
            children_map.insert(name.clone(), children.clone());
            for child in children {
                held_programs.insert(child);
            }
        }
        
        all_programs.insert(name);
    }
    
    // Find root
    let root = all_programs
        .into_iter()
        .find(|p| !held_programs.contains(p))
        .unwrap();
    
    (weights, children_map, root)
}

/// Calculate the total weight of a tower (node + all its descendants)
fn calculate_tower_weight(
    node: &str,
    weights: &HashMap<String, i32>,
    children_map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    if let Some(&weight) = memo.get(node) {
        return weight;
    }
    
    let mut total = weights[node];
    
    if let Some(children) = children_map.get(node) {
        for child in children {
            total += calculate_tower_weight(child, weights, children_map, memo);
        }
    }
    
    memo.insert(node.to_string(), total);
    total
}

/// Find the unbalanced node and return the corrected weight
fn find_unbalanced(
    node: &str,
    weights: &HashMap<String, i32>,
    children_map: &HashMap<String, Vec<String>>,
    tower_weights: &HashMap<String, i32>,
) -> Option<i32> {
    // Check if this node has children
    let children = match children_map.get(node) {
        Some(c) => c,
        None => return None, // Leaf node, can't be unbalanced
    };
    
    if children.is_empty() {
        return None;
    }
    
    // Calculate weights of each child's tower
    let child_tower_weights: Vec<(String, i32)> = children
        .iter()
        .map(|child| (child.clone(), tower_weights[child]))
        .collect();
    
    // Group by weight to find the odd one out
    let mut weight_counts: HashMap<i32, Vec<String>> = HashMap::new();
    for (child, weight) in &child_tower_weights {
        weight_counts
            .entry(*weight)
            .or_insert_with(Vec::new)
            .push(child.clone());
    }
    
    // If all weights are the same, this disc is balanced
    if weight_counts.len() == 1 {
        // Check children recursively
        for child in children {
            if let Some(result) = find_unbalanced(child, weights, children_map, tower_weights) {
                return Some(result);
            }
        }
        return None;
    }
    
    // Find the incorrect weight (the one that appears only once)
    let (incorrect_weight, incorrect_nodes) = weight_counts
        .iter()
        .find(|(_, nodes)| nodes.len() == 1)
        .unwrap();
    
    let incorrect_node = &incorrect_nodes[0];
    
    // Find the correct weight (the one that appears multiple times)
    let correct_weight = weight_counts
        .keys()
        .find(|&&w| w != *incorrect_weight)
        .copied()
        .unwrap();
    
    // First check if the problem is deeper in the tree
    if let Some(result) = find_unbalanced(incorrect_node, weights, children_map, tower_weights) {
        return Some(result);
    }
    
    // The problem is with this node itself
    let weight_diff = correct_weight - incorrect_weight;
    let current_weight = weights[incorrect_node];
    Some(current_weight + weight_diff)
}

/// Solve part 2: Find the weight needed to balance the tower
/// 
/// Approach: Calculate total tower weights for all nodes, then traverse the tree
/// to find the unbalanced disc. The unbalanced node is the one with children
/// where one child's tower weight differs from the others.
pub fn solve_part2(input: &str) -> i32 {
    let (weights, children_map, root) = build_tree(input);
    
    // Calculate total tower weight for each node
    let mut tower_weights = HashMap::new();
    let mut memo = HashMap::new();
    
    for node in weights.keys() {
        let weight = calculate_tower_weight(node, &weights, &children_map, &mut memo);
        tower_weights.insert(node.clone(), weight);
    }
    
    // Find the unbalanced node and return the corrected weight
    find_unbalanced(&root, &weights, &children_map, &tower_weights)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input;

    #[test]
    fn test_part1_examples() {
        let example = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        
        assert_eq!(solve_part1(example), "tknk");
    }

    #[test]
    fn test_part1_input() {
        let input =
            input::read_input("src/solutions/day07/input.txt").expect("Failed to read input file");
        let answer = solve_part1(&input);
        assert_eq!(answer, "eqgvf");
    }

    #[test]
    fn test_part2_examples() {
        let example = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        
        assert_eq!(solve_part2(example), 60);
    }

    #[test]
    fn test_part2_input() {
        let input =
            input::read_input("src/solutions/day07/input.txt").expect("Failed to read input file");
        let answer = solve_part2(&input);
        assert_eq!(answer, 757);
    }
}