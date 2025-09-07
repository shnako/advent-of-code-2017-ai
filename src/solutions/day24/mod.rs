// Day 24: Electromagnetic Moat

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Component {
    port1: i32,
    port2: i32,
}

impl Component {
    fn parse(line: &str) -> Component {
        let parts: Vec<&str> = line.split('/').collect();
        Component {
            port1: parts[0].parse().unwrap(),
            port2: parts[1].parse().unwrap(),
        }
    }

    fn strength(&self) -> i32 {
        self.port1 + self.port2
    }

    fn has_port(&self, port: i32) -> bool {
        self.port1 == port || self.port2 == port
    }

    fn other_port(&self, port: i32) -> Option<i32> {
        if self.port1 == port {
            Some(self.port2)
        } else if self.port2 == port {
            Some(self.port1)
        } else {
            None
        }
    }
}

fn find_strongest_bridge(
    components: &[Component],
    used: &mut HashSet<usize>,
    current_port: i32,
    current_strength: i32,
) -> i32 {
    let mut max_strength = current_strength;

    for (i, component) in components.iter().enumerate() {
        if !used.contains(&i) && component.has_port(current_port) {
            if let Some(next_port) = component.other_port(current_port) {
                used.insert(i);
                let strength = find_strongest_bridge(
                    components,
                    used,
                    next_port,
                    current_strength + component.strength(),
                );
                max_strength = max_strength.max(strength);
                used.remove(&i);
            }
        }
    }

    max_strength
}

fn find_longest_bridge(
    components: &[Component],
    used: &mut HashSet<usize>,
    current_port: i32,
    current_length: usize,
    current_strength: i32,
) -> (usize, i32) {
    let mut best = (current_length, current_strength);

    for (i, component) in components.iter().enumerate() {
        if !used.contains(&i) && component.has_port(current_port) {
            if let Some(next_port) = component.other_port(current_port) {
                used.insert(i);
                let result = find_longest_bridge(
                    components,
                    used,
                    next_port,
                    current_length + 1,
                    current_strength + component.strength(),
                );

                // Choose the longer bridge, or if equal length, the stronger one
                if result.0 > best.0 || (result.0 == best.0 && result.1 > best.1) {
                    best = result;
                }

                used.remove(&i);
            }
        }
    }

    best
}

pub fn solve_part1(input: &str) -> i32 {
    let components: Vec<Component> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Component::parse)
        .collect();

    let mut used = HashSet::new();
    find_strongest_bridge(&components, &mut used, 0, 0)
}

pub fn solve_part2(input: &str) -> i32 {
    let components: Vec<Component> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Component::parse)
        .collect();

    let mut used = HashSet::new();
    let (_, strength) = find_longest_bridge(&components, &mut used, 0, 0, 0);
    strength
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        assert_eq!(solve_part1(input), 31);
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day24/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 1859);
    }

    #[test]
    fn test_part2_examples() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

        // The longest bridge is length 4 with strength 19
        assert_eq!(solve_part2(input), 19);
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day24/input.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 1799);
    }
}
