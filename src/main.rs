mod error;
mod level;
mod program;
mod solution;

use level::LevelFile;
use program::Program;
use solution::Solution;

fn main() {
    env_logger::init();

    let solution = Solution::from_file("programs/02.hrm").unwrap();
    let mut prog = Program::new(&solution);

    let level = LevelFile::from_file("levels/01.level").unwrap();
    for test in &level.tests {
        match prog.run(&test.input) {
            Err(e) => panic!("{e}"),
            Ok((cycles, result)) => {
                let expected = &test.output;

                assert_eq!(result.len(), expected.len());
                expected.iter().zip(&result).for_each(|(expected, actual)| {
                    assert_eq!(expected, actual);
                });

                println!("Success!");
                println!("Outbox: {:?}", result);
                println!("Cycles: {cycles} [goal {}]", level.goals.speed);
                println!(
                    "Instructions: {} [goal {}]",
                    solution.len(),
                    level.goals.size
                );
            }
        }
    }
}
