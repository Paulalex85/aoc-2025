use crate::days::Day;
use winnow::ascii::{digit1, line_ending, multispace0, multispace1, space1};
use winnow::combinator::{opt, separated, terminated};
use winnow::token::any;
use winnow::{Parser, Result};

pub struct Day06;

pub struct MathExpression {
    numbers: Vec<Vec<u64>>,
    operators: Vec<char>,
}

fn parse_operators(input: &mut &str) -> Result<Vec<char>> {
    terminated(
        separated(
            1..,
            any.verify_map(|c| match c {
                '+' | '*' => Some(c),
                _ => None,
            }),
            multispace1,
        ),
        multispace0,
    )
    .parse_next(input)
}

fn parse_line_numbers(input: &mut &str) -> Result<Vec<u64>> {
    separated(1.., digit1.parse_to::<u64>(), space1).parse_next(input)
}

fn parse_all_numbers(input: &mut &str) -> Result<Vec<Vec<u64>>> {
    terminated(
        separated(1.., parse_line_numbers, line_ending),
        opt(line_ending),
    )
    .parse_next(input)
}

impl Day for Day06 {
    type Input = MathExpression;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let numbers = parse_all_numbers(input)?;
        let operators = parse_operators(input)?;

        Ok(MathExpression { numbers, operators })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result: usize = 0;
        for i in 0..input.numbers.get(0).unwrap().len() {
            result += input.numbers.iter().map(|number_line| number_line[i]).fold(
                if input.operators[i] == '+' { 0 } else { 1 },
                |acc, x| {
                    if input.operators[i] == '+' {
                        acc + x
                    } else {
                        acc * x
                    }
                },
            ) as usize
        }
        result
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
