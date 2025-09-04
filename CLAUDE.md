# Advent of Code 2017 Rust Assistant Instructions

You are helping solving the Advent of Code 2017 puzzles using Rust. This is a learning experience as the user has no Rust experience.

You will delegate all of your work to sub-agents.

## Advent of Code

This is a programming challenge that spreads over 25 days.

Every day a puzzle is released. You need to solve part 1 first and submit the answer. If this is correct, the part 2 is revealed. Once you solve this and submit the answer, the day's puzzle is considered solved, but we are not done yet as we still need to follow more steps to ensure a great solution is pushed into the GitHub repository.

You are solving the puzzle for year 2017. As we are currently in 2025, the puzzles for all days have been released - part 1 is available right away, while part 2 will be revealed after solving part 1.


## Workflow

When the user asks you to solve, you'll need to follow these steps exactly, without skipping any of the steps:

1. Switch back to the main branch and pull its latest version.
2. Figure out which is the puzzle that needs to be solved by looking at the existing implementations. For example, if the last solved puzzle is day 10 part 1, you will start at step 2. If you need to solve a part 2 first, start at step 4.
3. Use the aoc-puzzle-fetcher agent to download part 1 of the puzzle and input from the Advent of Code website in the correct format for the specified day.
4. Use the puzzle-solver agent to solve part 1 of the puzzle and submit the solution to the Advent of Code website.
5. Use the aoc-puzzle-fetcher agent to download part 2 of the puzzle.
6. Use the puzzle-solver agent to solve part 2 of the puzzle and submit the solution to the Advent of Code website.
7. Use the aoc-puzzle-fetcher agent to download the complete puzzle.
8. Use the puzzle-solution-augmenter to add all the necessary changes to make the solution ready to be pushed, and then create a PR for it.
9. Run the puzzle-review-watcher agent AS MANY TIMES AS NECESSARY, until it reports that the PR is ready to be merged. This can run for a long time - DO NOT STOP IT and DO NOT MOVE TO STEP 10 UNTIL IT REPORTS THE PR IS READY TO BE MERGED.
    9a. Every time the puzzle-review-watcher agent reports comments have been added to the PR, directly implement the CodeRabbit feedback.
    9b. Once the feedback is implemented, run the puzzle-review-watcher agent again.
10. Only after the puzzle-review-watcher reports that the PR is ready to be merged, go ahead and merge the PR using the GitHub CLI (gh).

## File Structure

advent-of-code-2017-ai/
  ├── Cargo.toml                          # Rust project manifest
  ├── Cargo.lock                          # Locked dependencies
  ├── README.md                           # Main README with progress table
  ├── CLAUDE.md                           # This instructions file
  ├── src/
  │   ├── main.rs                         # Main entry point for runner
  │   ├── lib.rs                          # Library root (exports utilities)
  │   ├── utils/
  │   │   ├── mod.rs                      # Module declaration
  │   │   ├── input.rs                    # Input parsing utilities
  │   │   ├── math.rs                     # Math utilities
  │   │   ├── grid.rs                     # 2D grid utilities
  │   │   └── graph.rs                    # Graph algorithms
  │   └── solutions/
  │       ├── day01/
  │       │   ├── mod.rs                  # Solution implementation
  │       │   ├── input.txt               # Puzzle input
  │       │   └── puzzle.txt              # Problem description
  │       ├── day02/
  │       │   └── ...
  │       └── day25/
  │           └── ...
  └── tools/
      ├── fetch/
      │   ├── Cargo.toml                  # Separate crate for fetcher
      │   └── src/
      │       └── main.rs                 # Fetches puzzle and input
      └── submit/
          ├── Cargo.toml                  # Separate crate for submitter
          └── src/
              └── main.rs                 # Submits answers

## Command Execution Guidelines

To minimize permission requests:
- Avoid piping or combining commands (e.g., avoid `command1 && command2` or `command1 | command2`)
- Run commands separately instead of chaining them
- Use simple, individual commands whenever possible
- Prefer direct commands over complex shell operations
- When multiple commands are needed, execute them as separate Bash tool calls
- Use `ls` command for listing directories (not `dir` or Windows-specific commands)
- Avoid conditional shell constructs like `if exist` - use simple commands like `ls` and check the output
