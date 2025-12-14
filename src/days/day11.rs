use crate::days::Day;
use std::collections::HashMap;
use winnow::ascii::{alphanumeric1, line_ending, space1};
use winnow::combinator::{opt, preceded, separated, terminated};
use winnow::token::literal;
use winnow::{Parser, Result};

pub struct Day11;

fn parse_line(input: &mut &str) -> Result<(String, Vec<String>)> {
    (
        alphanumeric1.map(str::to_string),
        preceded(
            literal(':'),
            preceded(
                space1,
                separated(1.., alphanumeric1.map(str::to_string), space1),
            ),
        ),
    )
        .parse_next(input)
}

impl Day for Day11 {
    type Input = HashMap<String, Vec<String>>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        terminated(separated(1.., parse_line, line_ending), opt(line_ending))
            .map(|lines: Vec<_>| lines.into_iter().collect())
            .parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut found = 0;
        let mut path_pile: Vec<Vec<String>> = vec![vec!["you".to_string()]];

        while let Some(path) = path_pile.pop() {
            let last = path.last().unwrap();
            let next_paths = input.get(last).unwrap();
            for next_path in next_paths {
                if next_path == "out" {
                    found += 1;
                } else if !path.contains(&next_path) {
                    let mut temp = path.clone();
                    temp.push(next_path.clone());
                    path_pile.push(temp);
                }
            }
        }
        found
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
