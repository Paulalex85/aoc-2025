use crate::days::Day;
use std::collections::HashSet;
use winnow::ascii::{digit1, line_ending, space0, space1};
use winnow::combinator::{delimited, opt, separated, terminated};
use winnow::token::take_while;
use winnow::{Parser, Result, seq};

pub struct Day10;

pub struct Machine {
    indicator_light: Vec<bool>,
    buttons_set: Vec<Vec<u16>>,
    joltages: Vec<u16>,
}

fn parse_indicator_light(input: &mut &str) -> Result<Vec<bool>> {
    delimited(
        '[',
        take_while(1.., ['.', '#']).map(|s: &str| s.chars().map(|c| c == '#').collect()),
        ']',
    )
    .parse_next(input)
}

fn parse_button_set(input: &mut &str) -> Result<Vec<u16>> {
    delimited('(', separated(1.., digit1.parse_to::<u16>(), ','), ')').parse_next(input)
}

fn parse_buttons_set(input: &mut &str) -> Result<Vec<Vec<u16>>> {
    (space0, separated(1.., parse_button_set, space1), space0)
        .map(|(_, buttons, _)| buttons)
        .parse_next(input)
}

fn parse_joltages(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', separated(1.., digit1.parse_to::<u16>(), ','), '}').parse_next(input)
}

fn parse_line(input: &mut &str) -> Result<Machine> {
    seq! {
        Machine {
            indicator_light: parse_indicator_light,
            buttons_set: parse_buttons_set,
            joltages: parse_joltages,
        }
    }
    .parse_next(input)
}

fn bool_to_u16(start: u16, bits: Vec<bool>) -> u16 {
    bits.iter()
        .enumerate()
        .fold(start, |acc, (i, &b)| acc | ((b as u16) << i))
}

fn vec_index_bool_to_u16(start: u16, update: Vec<u16>) -> u16 {
    update.iter().fold(start, |acc, &bit| acc ^ (1u16 << bit))
}

impl Day for Day10 {
    type Input = Vec<Machine>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        Ok(
            terminated(separated(1.., parse_line, line_ending), opt(line_ending))
                .parse_next(input)?,
        )
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|machine| {
                let nb_to_reach = bool_to_u16(0, machine.indicator_light.clone());
                let mut nb_iter = 0;
                let mut reached_numbers: HashSet<u16> = HashSet::new();
                reached_numbers.insert(0);

                while !reached_numbers.contains(&nb_to_reach) {
                    nb_iter += 1;

                    let mut new_reached_numbers: HashSet<u16> = HashSet::new();
                    for reached_number in reached_numbers.iter() {
                        for button in machine.buttons_set.iter() {
                            let new_number = vec_index_bool_to_u16(*reached_number, button.clone());
                            new_reached_numbers.insert(new_number);
                        }
                    }
                    reached_numbers = new_reached_numbers;
                }
                return nb_iter;
            })
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
