use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day05;

pub struct Inventory {
    fresh_id_range: Vec<(u64, u64)>,
    ingredient_id: Vec<u64>,
}

fn parse_id_range(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(digit1.parse_to(), '-', digit1.parse_to()).parse_next(input)
}

fn parse_ingredient_id(input: &mut &str) -> Result<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_id_ranges_section(input: &mut &str) -> Result<Vec<(u64, u64)>> {
    separated(1.., parse_id_range, line_ending).parse_next(input)
}

fn parse_ingredient_ids_section(input: &mut &str) -> Result<Vec<u64>> {
    terminated(
        separated(1.., parse_ingredient_id, line_ending),
        opt(line_ending),
    )
    .parse_next(input)
}

impl Day for Day05 {
    type Input = Inventory;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let (id_ranges, ingredient_ids) = separated_pair(
            parse_id_ranges_section,
            (line_ending, line_ending),
            parse_ingredient_ids_section,
        )
        .parse_next(input)?;

        Ok(Inventory {
            fresh_id_range: id_ranges,
            ingredient_id: ingredient_ids,
        })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .ingredient_id
            .iter()
            .filter(|id| {
                input
                    .fresh_id_range
                    .iter()
                    .any(|range| range.0 <= **id && id <= &&range.1)
            })
            .count()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
