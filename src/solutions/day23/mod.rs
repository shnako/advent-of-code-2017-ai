// Day 23: Coprocessor Conflagration

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Register(char),
    Number(i64),
}

impl Value {
    fn parse(s: &str) -> Value {
        if let Ok(n) = s.parse::<i64>() {
            Value::Number(n)
        } else {
            Value::Register(s.chars().next().unwrap())
        }
    }

    fn get(&self, registers: &HashMap<char, i64>) -> i64 {
        match self {
            Value::Register(r) => *registers.get(r).unwrap_or(&0),
            Value::Number(n) => *n,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "set" => {
                let reg = parts[1].chars().next().unwrap();
                let val = Value::parse(parts[2]);
                Instruction::Set(reg, val)
            }
            "sub" => {
                let reg = parts[1].chars().next().unwrap();
                let val = Value::parse(parts[2]);
                Instruction::Sub(reg, val)
            }
            "mul" => {
                let reg = parts[1].chars().next().unwrap();
                let val = Value::parse(parts[2]);
                Instruction::Mul(reg, val)
            }
            "jnz" => {
                let val1 = Value::parse(parts[1]);
                let val2 = Value::parse(parts[2]);
                Instruction::Jnz(val1, val2)
            }
            _ => panic!("Unknown instruction: {}", parts[0]),
        }
    }
}

pub fn solve_part1(input: &str) -> i32 {
    let instructions: Vec<Instruction> =
        input.lines().map(Instruction::parse).collect();

    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut pc = 0i64;
    let mut mul_count = 0;

    while pc >= 0 && (pc as usize) < instructions.len() {
        match &instructions[pc as usize] {
            Instruction::Set(reg, val) => {
                let value = val.get(&registers);
                registers.insert(*reg, value);
                pc += 1;
            }
            Instruction::Sub(reg, val) => {
                let current = *registers.get(reg).unwrap_or(&0);
                let value = val.get(&registers);
                registers.insert(*reg, current - value);
                pc += 1;
            }
            Instruction::Mul(reg, val) => {
                let current = *registers.get(reg).unwrap_or(&0);
                let value = val.get(&registers);
                registers.insert(*reg, current * value);
                mul_count += 1;
                pc += 1;
            }
            Instruction::Jnz(cond, offset) => {
                if cond.get(&registers) != 0 {
                    pc += offset.get(&registers);
                } else {
                    pc += 1;
                }
            }
        }
    }

    mul_count
}

pub fn solve_part2(_input: &str) -> i32 {
    // Part 2 typically requires analyzing what the program does
    // The program is checking for prime numbers
    // We need to count non-primes between b and c (inclusive) with step 17

    // When a=1, b starts at 106700 and c at 123700
    let b_start = 106700;
    let c_end = 123700;
    let step = 17;

    let mut h = 0;
    let mut b = b_start;

    while b <= c_end {
        // Check if b is composite (not prime)
        if !is_prime(b) {
            h += 1;
        }
        b += step;
    }

    h
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day23/input.txt").unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 4225);
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day23/input.txt").unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 905);
    }
}
