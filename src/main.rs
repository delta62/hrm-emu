mod error;
mod level;
mod program;
mod solution;

use error::ProgramError;
use level::{Level, Test};
use program::Program;
use solution::Solution;
use std::fs::read_to_string;

fn main() {
    env_logger::init();

    let solution = Solution::from_file("programs/02.hrm").unwrap();
    let mut prog = Program::new(&solution);
    let level: Level = read_to_string("levels/01.level").unwrap().parse().unwrap();

    for test in &level.tests {
        run(&level, &mut prog, &test).unwrap();
    }
}

fn run(level: &Level, prog: &mut Program, test: &Test) -> Result<(), ProgramError> {
    let (cycles, result) = prog.run(&test.input)?;

    assert_eq!(result.len(), test.output.len());
    test.output
        .iter()
        .zip(&result)
        .for_each(|(expected, actual)| {
            assert_eq!(expected, actual);
        });

    println!("Success!");
    println!("Outbox: {:?}", result);
    println!("Cycles: {cycles} [goal {}]", level.goals.speed);
    println!(
        "Instructions: {} [goal {}]",
        test.input.len(),
        level.goals.size
    );

    Ok(())
}
