// Day 18: Duet
// https://adventofcode.com/2017/day/18

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
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "snd" => Instruction::Snd(Value::parse(parts[1])),
                "set" => Instruction::Set(parts[1].chars().next().unwrap(), Value::parse(parts[2])),
                "add" => Instruction::Add(parts[1].chars().next().unwrap(), Value::parse(parts[2])),
                "mul" => Instruction::Mul(parts[1].chars().next().unwrap(), Value::parse(parts[2])),
                "mod" => Instruction::Mod(parts[1].chars().next().unwrap(), Value::parse(parts[2])),
                "rcv" => Instruction::Rcv(parts[1].chars().next().unwrap()),
                "jgz" => Instruction::Jgz(Value::parse(parts[1]), Value::parse(parts[2])),
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let instructions = parse_instructions(input);
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut pc = 0i64;
    let mut last_sound = 0i64;
    
    while pc >= 0 && (pc as usize) < instructions.len() {
        let idx = pc as usize;
        match &instructions[idx] {
            Instruction::Snd(val) => {
                last_sound = val.get(&registers);
            },
            Instruction::Set(reg, val) => {
                registers.insert(*reg, val.get(&registers));
            },
            Instruction::Add(reg, val) => {
                let current = *registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, current + val.get(&registers));
            },
            Instruction::Mul(reg, val) => {
                let current = *registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, current * val.get(&registers));
            },
            Instruction::Mod(reg, val) => {
                let current = *registers.get(reg).unwrap_or(&0);
                let divisor = val.get(&registers);
                if divisor != 0 {
                    registers.insert(*reg, current % divisor);
                }
            },
            Instruction::Rcv(reg) => {
                if *registers.get(reg).unwrap_or(&0) != 0 {
                    return last_sound.to_string();
                }
            },
            Instruction::Jgz(check, offset) => {
                if check.get(&registers) > 0 {
                    pc += offset.get(&registers) - 1; // -1 because we increment after
                }
            },
        }
        pc += 1;
    }
    
    "0".to_string()
}

pub fn solve_part2(input: &str) -> String {
    let instructions = parse_instructions(input);
    
    // Program state
    struct Program {
        _id: i64,
        registers: HashMap<char, i64>,
        pc: i64,
        send_queue: Vec<i64>,
        send_count: usize,
        waiting: bool,
    }
    
    let mut programs = vec![
        Program {
            _id: 0,
            registers: HashMap::from([('p', 0)]),
            pc: 0,
            send_queue: Vec::new(),
            send_count: 0,
            waiting: false,
        },
        Program {
            _id: 1,
            registers: HashMap::from([('p', 1)]),
            pc: 0,
            send_queue: Vec::new(),
            send_count: 0,
            waiting: false,
        },
    ];
    
    // Run until both programs are waiting or terminated
    loop {
        let mut any_progress = false;
        
        for prog_id in 0..2 {
            let other_id = 1 - prog_id;
            
            // Check if program can execute
            if programs[prog_id].pc < 0 || programs[prog_id].pc as usize >= instructions.len() {
                continue;
            }
            
            let idx = programs[prog_id].pc as usize;
            let mut increment_pc = true;
            
            match &instructions[idx] {
                Instruction::Rcv(reg) => {
                    // Try to receive from the other program's send queue
                    if !programs[other_id].send_queue.is_empty() {
                        let val = programs[other_id].send_queue.remove(0);
                        programs[prog_id].registers.insert(*reg, val);
                        programs[prog_id].waiting = false;
                        any_progress = true;
                    } else {
                        programs[prog_id].waiting = true;
                        increment_pc = false;
                    }
                },
                Instruction::Snd(val) => {
                    let value = val.get(&programs[prog_id].registers);
                    programs[prog_id].send_queue.push(value);
                    programs[prog_id].send_count += 1;
                    any_progress = true;
                },
                Instruction::Set(reg, val) => {
                    let value = val.get(&programs[prog_id].registers);
                    programs[prog_id].registers.insert(*reg, value);
                    any_progress = true;
                },
                Instruction::Add(reg, val) => {
                    let current = *programs[prog_id].registers.get(reg).unwrap_or(&0);
                    let value = val.get(&programs[prog_id].registers);
                    programs[prog_id].registers.insert(*reg, current + value);
                    any_progress = true;
                },
                Instruction::Mul(reg, val) => {
                    let current = *programs[prog_id].registers.get(reg).unwrap_or(&0);
                    let value = val.get(&programs[prog_id].registers);
                    programs[prog_id].registers.insert(*reg, current * value);
                    any_progress = true;
                },
                Instruction::Mod(reg, val) => {
                    let current = *programs[prog_id].registers.get(reg).unwrap_or(&0);
                    let divisor = val.get(&programs[prog_id].registers);
                    if divisor != 0 {
                        programs[prog_id].registers.insert(*reg, current % divisor);
                    }
                    any_progress = true;
                },
                Instruction::Jgz(check, offset) => {
                    if check.get(&programs[prog_id].registers) > 0 {
                        programs[prog_id].pc += offset.get(&programs[prog_id].registers) - 1;
                    }
                    any_progress = true;
                },
            }
            
            if increment_pc {
                programs[prog_id].pc += 1;
            }
        }
        
        // Check for deadlock or completion
        let prog0_terminated = programs[0].pc < 0 || programs[0].pc as usize >= instructions.len();
        let prog1_terminated = programs[1].pc < 0 || programs[1].pc as usize >= instructions.len();
        let prog0_waiting = programs[0].waiting && programs[1].send_queue.is_empty();
        let prog1_waiting = programs[1].waiting && programs[0].send_queue.is_empty();
        
        if !any_progress || (prog0_terminated || prog0_waiting) && (prog1_terminated || prog1_waiting) {
            break;
        }
    }
    
    programs[1].send_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(solve_part1(input), "4");
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("src/solutions/day18/input.txt").unwrap();
        assert_eq!(solve_part1(&input), "9423");
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("src/solutions/day18/input.txt").unwrap();
        assert_eq!(solve_part2(&input), "7620");
    }
}