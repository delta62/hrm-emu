use crate::{error::ParseError, program::Op};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::Path};

const HEADER_STR: &str = "-- HUMAN RESOURCE MACHINE PROGRAM --";

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct Solution {
    labels: HashMap<String, usize>,
    ops: Vec<Op>,
}

impl Solution {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        lazy_static! {
            static ref LABEL_PATTERN: Regex = Regex::new("[A-Za-z]+:").unwrap();
        }

        let s = read_to_string(path).map_err(ParseError::IoError)?;
        let mut lines = s.lines().filter(|line| !line.is_empty());
        let mut labels = HashMap::new();
        let mut ops = Vec::with_capacity(s.len());
        let mut op_num = 0;

        let header = lines.next().ok_or(ParseError::MissingHeader)?;
        if header != HEADER_STR {
            Err(ParseError::InvalidHeader)?;
        }

        for line in lines {
            let line = line.trim();

            if LABEL_PATTERN.is_match(line) {
                let label = line.split_once(':').unwrap().0.to_owned();
                labels.insert(label, op_num);
            } else {
                let op = line.parse()?;
                ops.push(op);
                op_num += 1;
            }
        }

        Ok(Self { labels, ops })
    }

    pub fn ops(&self) -> &[Op] {
        &self.ops
    }

    pub fn labels(&self) -> &HashMap<String, usize> {
        &self.labels
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }
}
