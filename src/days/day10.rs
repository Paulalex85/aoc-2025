use crate::days::Day;
use std::collections::HashSet;
use winnow::ascii::{digit1, line_ending, space0, space1};
use winnow::combinator::{delimited, opt, separated, terminated};
use winnow::token::take_while;
use winnow::{Parser, Result, seq};
use z3::ast::Int;
use z3::{Optimize, SatResult};

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

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|machine| {
                shortest_button_pressed_for_joltages(
                    machine.joltages.clone(),
                    machine.buttons_set.clone(),
                )
                .unwrap()
            })
            .sum()
    }
}

pub fn shortest_button_pressed_for_joltages(
    target: Vec<u16>,
    buttons: Vec<Vec<u16>>,
) -> Option<u64> {
    let opt = Optimize::new();

    let button_presses: Vec<_> = (0..buttons.len())
        .map(|i| Int::new_const(format!("btn_{}", i)))
        .collect();

    for press in &button_presses {
        opt.assert(&press.ge(&Int::from_i64(0)));
    }

    for pos in 0..target.len() {
        let mut sum = Int::from_i64(0);

        for (btn_idx, button) in buttons.iter().enumerate() {
            if button.contains(&(pos as u16)) {
                sum = Int::add(&[&sum, &button_presses[btn_idx]]);
            }
        }

        opt.assert(&sum.eq(&Int::from_i64(target[pos] as i64)));
    }

    let total = Int::add(&button_presses.iter().map(|x| x).collect::<Vec<_>>());
    opt.minimize(&total);

    if opt.check(&[]) == SatResult::Sat {
        if let Some(model) = opt.get_model() {
            let result = model.eval(&total, true).unwrap().as_i64().unwrap_or(0) as u64;
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let parsed = Day10::parser(&mut INPUT).unwrap();
        assert_eq!(Day10::part_2(&parsed), 33);
    }
}
