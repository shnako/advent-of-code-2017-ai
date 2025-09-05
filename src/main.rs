// Advent of Code 2017 - Solution Runner
// Runs all implemented solutions for testing and verification

use advent_of_code_2017::solutions;
use advent_of_code_2017::utils::input;
use anyhow::Result;
use std::env;
use std::time::Instant;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let day: u32 = args[1].parse().map_err(|_| anyhow::anyhow!("Invalid day number"))?;
        run_specific_day(day)?;
    } else {
        run_all_solutions()?;
    }
    
    Ok(())
}

fn run_all_solutions() -> Result<()> {
    println!("🎄 Advent of Code 2017 - Running All Solutions 🎄\n");
    
    // Day 1
    run_day(1)?;
    
    // Day 2
    run_day(2)?;
    
    // Day 3
    run_day(3)?;
    
    println!("\n✅ All solutions completed!");
    Ok(())
}

fn run_specific_day(day: u32) -> Result<()> {
    println!("🎄 Advent of Code 2017 - Running Day {} 🎄\n", day);
    run_day(day)?;
    Ok(())
}

fn run_day(day: u32) -> Result<()> {
    match day {
        1 => run_day_1(),
        2 => run_day_2(),
        3 => run_day_3(),
        _ => {
            println!("❌ Day {} not implemented yet", day);
            Ok(())
        }
    }
}

fn run_day_1() -> Result<()> {
    println!("📅 Day 1: Inverse Captcha");
    
    let input_path = "src/solutions/day01/input.txt";
    let input = input::read_input(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input for day 1: {}", e))?;
    
    // Part 1
    let start = Instant::now();
    let part1_result = solutions::day01::solve_part1(&input);
    let part1_duration = start.elapsed();
    println!("  Part 1: {} ({}µs)", part1_result, part1_duration.as_micros());
    
    // Part 2
    let start = Instant::now();
    let part2_result = solutions::day01::solve_part2(&input);
    let part2_duration = start.elapsed();
    println!("  Part 2: {} ({}µs)", part2_result, part2_duration.as_micros());
    
    println!("  ✅ Day 1 completed!\n");
    Ok(())
}

fn run_day_2() -> Result<()> {
    println!("📅 Day 2: Corruption Checksum");
    
    let input_path = "src/solutions/day02/input.txt";
    let input = input::read_input(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input for day 2: {}", e))?;
    
    // Part 1
    let start = Instant::now();
    let part1_result = solutions::day02::solve_part1(&input);
    let part1_duration = start.elapsed();
    println!("  Part 1: {} ({}µs)", part1_result, part1_duration.as_micros());
    
    // Part 2
    let start = Instant::now();
    let part2_result = solutions::day02::solve_part2(&input);
    let part2_duration = start.elapsed();
    println!("  Part 2: {} ({}µs)", part2_result, part2_duration.as_micros());
    
    println!("  ✅ Day 2 completed!\n");
    Ok(())
}

fn run_day_generic(
    title: &str,
    input_path: &str,
    solve1: fn(&str) -> i32,
    solve2: fn(&str) -> i32,
    day_num: u32,
) -> Result<()> {
    println!("{}", title);
    let input = input::read_input(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read input for day {}: {}", day_num, e))?;

    let start = Instant::now();
    let p1 = solve1(&input);
    let d1 = start.elapsed();
    println!("  Part 1: {} ({}µs)", p1, d1.as_micros());

    let start = Instant::now();
    let p2 = solve2(&input);
    let d2 = start.elapsed();
    println!("  Part 2: {} ({}µs)", p2, d2.as_micros());

    println!("  ✅ Day {} completed!\n", day_num);
    Ok(())
}

fn run_day_3() -> Result<()> {
    run_day_generic(
        "📅 Day 3: Spiral Memory",
        "src/solutions/day03/input.txt",
        solutions::day03::solve_part1,
        solutions::day03::solve_part2,
        3,
    )
}