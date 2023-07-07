use std::{fmt::Display, io};

pub type Result<T> = std::result::Result<T, ProgramError>;

#[derive(Debug)]
pub enum ProgramError {
    EmptyAccumulator,
    EndOfInput,
    MaxCyclesExceeded,
    UndefinedLabel(String),
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProgramError::*;

        match self {
            EmptyAccumulator => write!(f, "Attempted to run an instruction that reads from the accumulator, but the accumulator is empty"),
            EndOfInput => write!(f, "Attempted to read past the end of the input"),
            MaxCyclesExceeded => write!(f, "Maximum cycle count exceeded"),
            UndefinedLabel(label) => write!(f, "Undefined label \"{label}\""),
        }
    }
}

impl std::error::Error for ProgramError {}

#[derive(Debug)]
pub enum ParseError {
    InvalidHeader,
    IoError(io::Error),
    MissingHeader,
    UnexpectedToken(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;

        match self {
            InvalidHeader => write!(f, "Invalid program header"),
            IoError(e) => write!(f, "{e}"),
            MissingHeader => write!(f, "Program must start with a HRM header"),
            UnexpectedToken(t) => write!(f, "Unexpected token \"{t}\""),
        }
    }
}

impl std::error::Error for ParseError {}
