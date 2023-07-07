use std::{collections::HashMap, str::FromStr};

use crate::{
    error::{ParseError, ProgramError, Result},
    solution::Solution,
};

const CYCLE_LIMIT: usize = 1000;

#[derive(Debug)]
pub enum Op {
    Inbox,
    Outbox,
    Jump(String),
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Op::*;

        let mut tokens = s.split_whitespace();
        let ret = match tokens.next().unwrap() {
            "INBOX" => Inbox,
            "OUTBOX" => Outbox,
            "JUMP" => {
                let destination = tokens.next().unwrap().to_owned();
                Jump(destination)
            }
            _ => Err(ParseError::UnexpectedToken(s.to_owned()))?,
        };

        if let Some(extra) = tokens.next() {
            Err(ParseError::UnexpectedToken(extra.to_owned()))
        } else {
            Ok(ret)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    // Char(char),
    Int(i16),
}

#[derive(Debug)]
pub struct Program<'a> {
    acc: Option<Value>,
    cy: usize,
    labels: &'a HashMap<String, usize>,
    ops: &'a [Op],
    pc: usize,
    output: Vec<Value>,
}

impl<'a> Program<'a> {
    pub fn new(solution: &'a Solution) -> Self {
        let acc = None;
        let cy = 0;
        let pc = 0;
        let output = Vec::new();
        let ops = solution.ops();
        let labels = solution.labels();

        Self {
            acc,
            cy,
            labels,
            ops,
            output,
            pc,
        }
    }

    fn reset(&mut self) {
        self.acc = None;
        self.cy = 0;
        self.output = Vec::new();
        self.pc = 0;
    }

    pub fn run(&mut self, input: &[Value]) -> Result<(usize, Vec<Value>)> {
        let mut input = input.into_iter();
        self.reset();

        while let Some(op) = self.ops.get(self.pc) {
            log::debug!("Next op: {:?}", op);

            match op {
                Op::Inbox => self.inbox(&mut input)?,
                Op::Outbox => self.outbox()?,
                Op::Jump(d) => self.jump(d)?,
            }

            self.cy += 1;

            if self.cy > CYCLE_LIMIT {
                Err(ProgramError::MaxCyclesExceeded)?;
            }
        }

        let output = std::mem::take(&mut self.output);
        Ok((self.cy, output))
    }

    fn inbox<'i>(&mut self, mut input: impl Iterator<Item = &'i Value>) -> Result<()> {
        let next_input = *input.next().ok_or(ProgramError::EndOfInput)?;
        log::debug!("next input: {:?}", next_input);

        self.acc = Some(next_input);
        self.pc += 1;

        Ok(())
    }

    fn outbox(&mut self) -> Result<()> {
        let val = self.acc.ok_or(ProgramError::EmptyAccumulator)?;
        log::debug!("output: {:?}", val);

        self.output.push(val);
        self.acc = None;
        self.pc += 1;

        Ok(())
    }

    fn jump(&mut self, to: &str) -> Result<()> {
        log::debug!("jump to {to}");
        let label_location = self
            .labels
            .get(to)
            .ok_or_else(|| ProgramError::UndefinedLabel(to.to_owned()))?;

        self.pc = *label_location;

        Ok(())
    }
}
