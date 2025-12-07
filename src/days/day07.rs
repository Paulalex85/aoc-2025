use crate::days::Day;
use std::collections::BTreeSet;
use winnow::ascii::line_ending;
use winnow::combinator::{opt, separated, terminated};
use winnow::token::take_while;
use winnow::{Parser, Result};

pub struct Day07;

fn parse_line(input: &mut &str) -> Result<String> {
    take_while(1.., |c: char| c != '\n' && c != '\r')
        .map(|s: &str| s.to_string())
        .parse_next(input)
}

impl Day for Day07 {
    type Input = Vec<String>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        Ok(
            terminated(separated(1.., parse_line, line_ending), opt(line_ending))
                .parse_next(input)?,
        )
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut set = BTreeSet::new();
        let mut result = 0;
        input.iter().for_each(|line| {
            line.char_indices()
                .filter(|(_, c)| *c == 'S' || *c == '^')
                .for_each(|(i, c)| {
                    if c == 'S' {
                        set.insert(i);
                    } else if c == '^' {
                        if set.contains(&i) {
                            set.remove(&i);
                            set.insert(i - 1);
                            set.insert(i + 1);
                            result += 1;
                        }
                    }
                })
        });
        result
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
