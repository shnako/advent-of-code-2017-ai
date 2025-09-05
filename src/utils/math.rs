// Math utilities for AOC puzzles

/// Greatest common divisor
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least common multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}
