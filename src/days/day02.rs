use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day02;

pub struct Id {
    value: (u64, u64),
}

fn parse_id(input: &mut &str) -> Result<Id> {
    separated_pair(digit1.parse_to(), '-', digit1.parse_to())
        .map(|x| Id { value: x })
        .parse_next(input)
}

impl Day for Day02 {
    type Input = Vec<Id>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let lines: Vec<Id> =
            terminated(separated(1.., parse_id, ','), multispace0).parse_next(input)?;
        Ok(lines)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .into_iter()
            .map(|ids| {
                let values: Vec<u64> = (ids.value.0..=ids.value.1).collect();
                values
                    .iter()
                    .filter(|x| {
                        let x = x.to_string();
                        x.len() % 2 == 0 && &x[0..x.len() / 2] == &x[x.len() / 2..]
                    })
                    .map(|x| *x as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .into_iter()
            .map(|ids| {
                let values: Vec<u64> = (ids.value.0..=ids.value.1).collect();
                values
                    .iter()
                    .filter(|x| {
                        let x = x.to_string();

                        for i in 1..=x.len() / 2 {
                            if x.len() % i != 0 {
                                continue;
                            }
                            if (1..x.len() / i).all(|j| &x[0..i] == &x[j * i..=((j + 1) * i) - 1]) {
                                return true;
                            }
                        }
                        return false;
                    })
                    .map(|x| *x as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let parsed = Day02::parser(&mut INPUT).unwrap();
        assert_eq!(Day02::part_1(&parsed), 1227775554);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let parsed = Day02::parser(&mut INPUT).unwrap();
        assert_eq!(Day02::part_2(&parsed), 4174379265);
    }
}
