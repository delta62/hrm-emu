use crate::error::ParseError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending, multispace0, space0, space1},
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::{pair, preceded, terminated, tuple, Tuple},
    Finish, IResult,
};
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

struct Label<'s>(&'s str);

struct KeyValue<'s>(&'s str, usize);

struct Sequence(Vec<usize>);

struct TestCase {
    input: Sequence,
    output: Sequence,
}

enum Statement<'s> {
    TestCase(TestCase),
    KeyValue(KeyValue<'s>),
}

struct Section<'s> {
    label: Label<'s>,
    statements: Vec<Statement<'s>>,
}

const NO_TEST_CASES: &Vec<Statement<'static>> = &Vec::new();

struct ParseLevel<'s>(HashMap<&'s str, Vec<Statement<'s>>>);

impl<'s> ParseLevel<'s> {
    fn get_value(&self, section: &str, key: &str) -> Option<usize> {
        self.0.get(section).and_then(|stmts| {
            stmts.iter().find_map(|stmt| match stmt {
                Statement::KeyValue(kv) if kv.0 == key => Some(kv.1),
                _ => None,
            })
        })
    }

    fn tests(&self, section: &str) -> impl Iterator<Item = &TestCase> {
        self.0
            .get(section)
            .unwrap_or(NO_TEST_CASES)
            .iter()
            .filter_map(|stmt| match stmt {
                Statement::TestCase(tc) => Some(tc),
                _ => None,
            })
    }
}

fn size(input: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(input, 10)
}

fn kvp(input: &str) -> IResult<&str, KeyValue<'_>> {
    let (input, res) = (alpha1, space0, char('='), space0, map_res(digit1, size)).parse(input)?;
    let pair = KeyValue(res.0, res.4);
    Ok((input, pair))
}

fn number_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(space1, map_res(digit1, size))(input)
}

fn sequence(input: &str) -> IResult<&str, Sequence> {
    let (input, res) = (char('['), space0, number_list, space0, char(']')).parse(input)?;
    Ok((input, Sequence(res.2)))
}

fn test_case(input: &str) -> IResult<&str, TestCase> {
    let (input, res) = (sequence, space0, tag("->"), space0, sequence).parse(input)?;
    let tc = TestCase {
        input: res.0,
        output: res.4,
    };

    Ok((input, tc))
}

fn label(input: &str) -> IResult<&str, Label<'_>> {
    preceded(
        multispace0,
        terminated(map(alpha1, Label), tuple((char(':'), line_ending))),
    )(input)
}

fn statement(input: &str) -> IResult<&str, Statement<'_>> {
    preceded(
        multispace0,
        terminated(
            alt((
                map(kvp, Statement::KeyValue),
                map(test_case, Statement::TestCase),
            )),
            line_ending,
        ),
    )(input)
}

fn section(input: &str) -> IResult<&str, Section<'_>> {
    map(pair(label, many0(statement)), |(label, statements)| {
        Section { label, statements }
    })(input)
}

fn level(input: &str) -> IResult<&str, ParseLevel<'_>> {
    let (input, sections) = terminated(many0(section), multispace0)(input)?;

    let mut lvl = HashMap::with_capacity(sections.len());
    for section in sections {
        let key = section.label.0;
        if lvl.contains_key(key) {
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::OneOf,
            )))?;
        }

        lvl.insert(key, section.statements);
    }

    let level = ParseLevel(lvl);
    Ok((input, level))
}

#[derive(Debug, Default)]
pub struct Test {
    pub input: Vec<usize>,
    pub output: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Goals {
    pub size: usize,
    pub speed: usize,
}

#[derive(Debug, Default)]
pub struct Level {
    pub goals: Goals,
    pub tests: Vec<Test>,
}

impl FromStr for Level {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, level) = level(s)
            .finish()
            .map_err(|_| ParseError::UnexpectedToken("x".to_owned()))?;

        if !input.is_empty() {
            Err(ParseError::UnexpectedToken(input.to_owned()))?;
        }

        let size = level.get_value("goals", "size").unwrap();
        let speed = level.get_value("goals", "speed").unwrap();
        let goals = Goals { size, speed };

        let tests = level
            .tests("tests")
            .map(|t| Test {
                input: t.input.0.clone(),
                output: t.output.0.clone(),
            })
            .collect();

        Ok(Level { goals, tests })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type TestResult = Result<(), nom::Err<nom::error::Error<&'static str>>>;

    macro_rules! parse_all {
        ($fn:expr, $input:literal) => {{
            let (rest, result) = $fn($input)?;
            assert!(rest.is_empty(), "Some input was unused: {rest}");
            result
        }};
    }

    #[test]
    fn parses_number() -> Result<(), ParseIntError> {
        let res = size("42")?;
        assert_eq!(res, 42);

        Ok(())
    }

    #[test]
    fn parses_label() -> TestResult {
        let label = parse_all!(label, "tests:\n");
        assert_eq!(label.0, "tests");
        Ok(())
    }

    #[test]
    fn parses_number_list() -> TestResult {
        let list = parse_all!(number_list, "1 2 3");
        assert_eq!(list, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn parses_sequence() -> TestResult {
        let list = parse_all!(sequence, "[ 1 2 3 ]");
        assert_eq!(list.0, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn parses_test_case() -> TestResult {
        let test = parse_all!(test_case, "[ 1 2 3 ] -> [ 3 2 1 ]");

        assert_eq!(test.input.0, vec![1, 2, 3]);
        assert_eq!(test.output.0, vec![3, 2, 1]);

        Ok(())
    }

    #[test]
    fn parses_key_value() -> TestResult {
        let pair = parse_all!(kvp, "key = 42");

        assert_eq!(pair.0, "key");
        assert_eq!(pair.1, 42);

        Ok(())
    }
}
