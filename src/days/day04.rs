use crate::days::Day;
use winnow::ascii::line_ending;
use winnow::combinator::{opt, separated, terminated};
use winnow::token::take_while;
use winnow::{Parser, Result};

pub struct Day04;

fn parse_line(input: &mut &str) -> Result<Vec<char>> {
    take_while(1.., |c: char| c == '@' || c == '.')
        .map(|s: &str| s.chars().collect())
        .parse_next(input)
}

impl Day for Day04 {
    type Input = Vec<Vec<char>>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        Ok(
            terminated(separated(1.., parse_line, line_ending), opt(line_ending))
                .parse_next(input)?,
        )
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut input_copy = input.clone();
        count_and_remove_rolls(&mut input_copy)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut input_copy = input.clone();
        let mut result = 0;
        let mut last_result = 1;
        while last_result > 0 {
            last_result = count_and_remove_rolls(&mut input_copy);
            result += last_result;
        }
        result
    }
}

fn count_and_remove_rolls(input: &mut Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let mut rolls_to_remove: Vec<(usize, usize)> = vec![];
    let width = input.get(0).unwrap().len();
    let height = input.len();
    for i in 0..height {
        for j in 0..width {
            if input.get(i).unwrap().get(j).unwrap() != &'@' {
                continue;
            }
            let mut count = 0;
            if j > 0 && i > 0 && input.get(i - 1).unwrap().get(j - 1).unwrap() == &'@' {
                count += 1;
            }
            if i > 0 && input.get(i - 1).unwrap().get(j).unwrap() == &'@' {
                count += 1;
            }
            if j < width - 1 && i > 0 && input.get(i - 1).unwrap().get(j + 1).unwrap() == &'@' {
                count += 1;
            }
            if j > 0 && input.get(i).unwrap().get(j - 1).unwrap() == &'@' {
                count += 1;
            }
            if j < width - 1 && input.get(i).unwrap().get(j + 1).unwrap() == &'@' {
                count += 1;
            }
            if j > 0 && i < height - 1 && input.get(i + 1).unwrap().get(j - 1).unwrap() == &'@' {
                count += 1;
            }
            if i < height - 1 && input.get(i + 1).unwrap().get(j).unwrap() == &'@' {
                count += 1;
            }
            if j < width - 1
                && i < height - 1
                && input.get(i + 1).unwrap().get(j + 1).unwrap() == &'@'
            {
                count += 1;
            }
            if count < 4 {
                result += 1;
                rolls_to_remove.push((i, j));
            }
        }
    }

    for roll in rolls_to_remove {
        input[roll.0][roll.1] = '.';
    }

    result
}
