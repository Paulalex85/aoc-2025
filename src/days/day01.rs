use crate::days::Day;
use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, terminated};
use winnow::token::any;
use winnow::{Parser, Result, seq};

pub struct Day01;

enum Direction {
    Left,
    Right,
}

pub struct Rotation {
    direction: Direction,
    steps: u32,
}

fn parse_direction(input: &mut &str) -> Result<Direction> {
    any.verify_map(|c| match c {
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Right),
        _ => None,
    })
    .parse_next(input)
}

fn parse_rotation(input: &mut &str) -> Result<Rotation> {
    seq! {
        Rotation {
            direction: parse_direction,
            steps: digit1.parse_to(),
        }
    }
    .parse_next(input)
}

impl Day for Day01 {
    type Input = Vec<Rotation>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        Ok(terminated(
            separated(1.., parse_rotation, line_ending),
            opt(line_ending),
        )
        .parse_next(input)?)
    }

    type Output1 = usize;

    //126.833µs
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut count: u32 = 0;
        let mut dial: i32 = 50;
        for rotation in input.iter() {
            match rotation.direction {
                Direction::Left => {
                    dial += rotation.steps as i32;
                }
                Direction::Right => {
                    dial -= rotation.steps as i32;
                }
            }

            if dial % 100 == 0 {
                count += 1;
            }
        }

        count as usize
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut count: u32 = 0;
        let mut dial: u32 = 50;
        for rotation in input.iter() {
            let prev_dial = dial;
            match rotation.direction {
                Direction::Left => {
                    let current_remaining_steps = 100 - dial;
                    if current_remaining_steps <= rotation.steps {
                        let remaining_steps = rotation.steps - current_remaining_steps;
                        let rotations = remaining_steps / 100;
                        dial = remaining_steps % 100;
                        if dial == 0 {
                            count += rotations;
                        } else {
                            count += rotations + 1;
                        }
                    } else {
                        dial += rotation.steps;
                    }
                }
                Direction::Right => {
                    let current_remaining_steps = dial;
                    if current_remaining_steps < rotation.steps {
                        let remaining_steps = rotation.steps - current_remaining_steps;
                        let mut rotations = remaining_steps / 100;
                        let was_on_zero = dial == 0;
                        dial = (100 - remaining_steps % 100) % 100;
                        if dial != 0 {
                            rotations += 1;
                        }
                        if was_on_zero {
                            rotations -= 1;
                        }
                        count += rotations;
                    } else {
                        dial -= rotation.steps;
                    }
                }
            }
            if dial == 0 {
                count += 1;
            }
            println!("{} {} {} {}", count, dial, rotation.steps, prev_dial);
        }

        count as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let parsed = Day01::parser(&mut INPUT).unwrap();
        assert_eq!(Day01::part_2(&parsed), 6);
    }
}
