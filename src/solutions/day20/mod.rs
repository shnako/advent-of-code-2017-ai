// Day 20: Particle Swarm
// https://adventofcode.com/2017/day/20

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn add(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone)]
struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
}

impl Particle {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split(", ").collect();

        fn parse_vector(s: &str) -> Vector3 {
            let s = s
                .trim_start_matches("p=<")
                .trim_start_matches("v=<")
                .trim_start_matches("a=<")
                .trim_end_matches('>');
            let coords: Vec<i64> = s.split(',').map(|n| n.parse().unwrap()).collect();
            Vector3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        }

        Particle {
            position: parse_vector(parts[0]),
            velocity: parse_vector(parts[1]),
            acceleration: parse_vector(parts[2]),
        }
    }

    fn tick(&mut self) {
        self.velocity.add(&self.acceleration);
        self.position.add(&self.velocity);
    }
}

pub fn solve_part1(input: &str) -> String {
    let particles: Vec<Particle> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Particle::parse)
        .collect();

    // The particle that stays closest in the long term is the one with the smallest acceleration
    // If tied, then smallest velocity, then smallest initial position
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| {
            (
                p.acceleration.manhattan(),
                p.velocity.manhattan(),
                p.position.manhattan(),
            )
        })
        .unwrap()
        .0
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut particles: Vec<Particle> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(Particle::parse)
        .collect();

    let mut alive: Vec<bool> = vec![true; particles.len()];

    // Simulate for enough ticks to let collisions happen
    for _ in 0..1000 {
        // Update positions
        for (i, particle) in particles.iter_mut().enumerate() {
            if alive[i] {
                particle.tick();
            }
        }

        // Check for collisions
        let mut position_map: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
        for (i, particle) in particles.iter().enumerate() {
            if alive[i] {
                let pos = (
                    particle.position.x,
                    particle.position.y,
                    particle.position.z,
                );
                position_map.entry(pos).or_insert_with(Vec::new).push(i);
            }
        }

        // Remove collided particles
        for indices in position_map.values() {
            if indices.len() > 1 {
                for &i in indices {
                    alive[i] = false;
                }
            }
        }
    }

    alive.iter().filter(|&&a| a).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        assert_eq!(solve_part1(input), "0");
    }
}
