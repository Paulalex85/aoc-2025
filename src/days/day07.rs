use crate::days::Day;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
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

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut paths: HashMap<usize, u64> = HashMap::new();

        input.iter().for_each(|line| {
            if paths.is_empty() {
                let start = line.find('S');
                if start.is_some() {
                    paths.insert(start.unwrap(), 1);
                }
            } else {
                paths
                    .clone()
                    .iter()
                    .sorted()
                    .for_each(|(index_path, value)| {
                        if line.chars().nth(*index_path).unwrap() == '^' {
                            paths.insert(
                                *index_path - 1,
                                *value + paths.get(&(index_path - 1)).unwrap_or(&0),
                            );
                            paths.insert(
                                *index_path + 1,
                                *value + paths.get(&(index_path + 1)).unwrap_or(&0),
                            );
                            paths.remove(index_path);
                        }
                    })
            }
        });

        paths.values().copied().sum::<u64>() as usize
    }
}
