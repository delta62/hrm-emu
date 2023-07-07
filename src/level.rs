use crate::{error::ParseError, program::Value};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::read_to_string, path::Path, str::FromStr};

#[derive(Debug)]
pub struct Test {
    pub input: Vec<Value>,
    pub output: Vec<Value>,
}

#[derive(Debug)]
pub struct Goals {
    pub size: usize,
    pub speed: usize,
}

#[derive(Debug)]
pub struct LevelFile {
    pub tests: Vec<Test>,
    pub goals: Goals,
}

enum Section {
    Tests,
    Goals,
}

impl FromStr for Section {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tests:" => Ok(Self::Tests),
            "goals:" => Ok(Self::Goals),
            _ => Err(ParseError::UnexpectedToken(s.to_owned())),
        }
    }
}

impl LevelFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let s = read_to_string(path).map_err(ParseError::IoError)?;
        s.parse()
    }
}

impl FromStr for LevelFile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SECTION_PATTERN: Regex = Regex::new("^[A-Za-z]:$").unwrap();
            static ref IO_PATTERN: Regex = Regex::new(r"^\[\s*(?:(\d+),\s*)+\]$").unwrap();
        }

        let lines = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim());

        let mut tests = Vec::new();
        let mut goals = Goals { size: 0, speed: 0 };
        let mut current_section: Option<Section> = None;

        for line in lines {
            println!("Line '{line}'");
            if SECTION_PATTERN.is_match(line) {
                println!("section match");
                current_section = Some(line.parse()?);
                continue;
            }

            println!("non-section match");

            match current_section {
                Some(Section::Tests) => {
                    // [ 1, 2, 3 ] -> [ 4, 5, 6 ]
                    let (input, output) = line.split_once("->").unwrap();
                    let input = IO_PATTERN.captures(input).unwrap();
                    println!("{:?}", input);
                    todo!();
                }
                Some(Section::Goals) => {
                    let mut tokens = line.split('=').map(|t| t.trim());
                    match tokens.next() {
                        Some("size") => goals.size = tokens.next().unwrap().parse().unwrap(),
                        Some("speed") => goals.speed = tokens.next().unwrap().parse().unwrap(),
                        Some(_) => unimplemented!(),
                        None => unimplemented!(),
                    }
                }
                None => Err(ParseError::UnexpectedToken(line.to_owned()))?,
            }
        }

        Ok(Self { tests, goals })
    }
}
