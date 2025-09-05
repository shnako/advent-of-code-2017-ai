// Graph algorithms and utilities

use std::collections::{HashMap, HashSet, VecDeque};

/// Basic graph representation
pub type Graph<T> = HashMap<T, Vec<T>>;

/// Breadth-first search
pub fn bfs<T: Clone + Eq + std::hash::Hash>(
    graph: &Graph<T>,
    start: &T,
    target: &T,
) -> Option<Vec<T>> {
    if start == target {
        return Some(vec![start.clone()]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent = HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    parent.insert(neighbor.clone(), current.clone());
                    queue.push_back(neighbor.clone());

                    if neighbor == target {
                        // Reconstruct path
                        let mut path = Vec::new();
                        let mut current = target.clone();
                        while let Some(p) = parent.get(&current) {
                            path.push(current);
                            current = p.clone();
                        }
                        path.push(start.clone());
                        path.reverse();
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Find all nodes reachable from a starting node using BFS
pub fn find_reachable<T: Clone + Eq + std::hash::Hash>(
    graph: &Graph<T>,
    start: &T,
) -> HashSet<T> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start.clone());
    visited.insert(start.clone());
    
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    
    visited
}
