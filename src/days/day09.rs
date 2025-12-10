use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day09;

fn parse_line(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(digit1.parse_to(), ',', digit1.parse_to()).parse_next(input)
}

impl Day for Day09 {
    type Input = Vec<(u64, u64)>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        terminated(separated(1.., parse_line, line_ending), opt(line_ending)).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| {
                input
                    .iter()
                    .skip(i)
                    .map(|&(x2, y2)| (x.abs_diff(x2) + 1) * (y.abs_diff(y2) + 1))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap() as usize
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
