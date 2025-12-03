use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day03;

fn parse_line(input: &mut &str) -> Result<String> {
    digit1.parse_to().parse_next(input)
}

impl Day for Day03 {
    type Input = Vec<String>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        Ok(
            terminated(separated(1.., parse_line, line_ending), opt(line_ending))
                .parse_next(input)?,
        )
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .into_iter()
            .map(|x| {
                let (index, max) = x[0..x.len() - 1]
                    .chars()
                    .enumerate()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|(i, c)| (i, c.to_digit(10).unwrap() as u8))
                    .max_by_key(|(_, c)| *c)
                    .unwrap();

                let second_digits = x[index + 1..]
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .max()
                    .unwrap();
                // println!("{} {} {}", max, second_digits, x);
                (max.to_string() + &second_digits.to_string())
                    .parse::<usize>()
                    .unwrap()
            })
            .sum::<usize>()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}

#[test]
fn test_part1() {
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    let parsed = Day03::parser(&mut INPUT).unwrap();
    assert_eq!(Day03::part_1(&parsed), 357);
}
