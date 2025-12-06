use crate::days::Day;
use itertools::Itertools;
use winnow::ascii::{digit1, line_ending, multispace0, multispace1, space1};
use winnow::combinator::{opt, separated, terminated};
use winnow::token::any;
use winnow::{Parser, Result};

pub struct Day06;

pub struct MathExpression {
    numbers: Vec<Vec<u64>>,
    operators: Vec<char>,
    raw_lines: Vec<String>,
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
        let start = *input;

        let numbers = parse_all_numbers(input)?;
        let operators = parse_operators(input)?;

        let raw_lines: Vec<String> = start
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|s| s.to_string())
            .collect();

        Ok(MathExpression {
            numbers,
            operators,
            raw_lines,
        })
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

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut result: usize = 0;
        let mut current_operator: Option<char> = None;
        let mut current_value: Option<u64> = None;
        for i in 0..input.raw_lines.get(0).unwrap().chars().count() {
            let column: Vec<char> = input
                .raw_lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap_or(' '))
                .collect();
            if column.iter().all(|x| *x == ' ') {
                result += current_value.unwrap_or(0) as usize;
                current_value = None;
            } else {
                if column.last().unwrap() != &' ' {
                    current_operator = Some(*column.last().unwrap());
                }
                let number: u64 = column[0..column.len() - 1]
                    .iter()
                    .filter(|x| *x != &' ')
                    .join("")
                    .parse::<u64>()
                    .unwrap_or(0);
                current_value = Some(match current_operator {
                    Some('+') => current_value.unwrap_or(0) + number,
                    Some('*') => current_value.unwrap_or(1) * number,
                    _ => 0,
                });
            }
        }
        result += current_value.unwrap_or(0) as usize;
        result
    }
}
