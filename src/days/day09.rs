use itertools::Itertools;
use std::collections::HashMap;
use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day09;

fn parse_line(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(digit1.parse_to(), ',', digit1.parse_to()).parse_next(input)
}

impl Day for Day09 {
    type Input = Vec<(u64, u64)>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        terminated(separated(1.., parse_line, line_ending), opt(line_ending)).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| {
                input
                    .iter()
                    .skip(i)
                    .map(|&(x2, y2)| (x.abs_diff(x2) + 1) * (y.abs_diff(y2) + 1))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap() as usize
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut biggest_valid_rectangle: u64 = 0;
        let mut tiles: HashMap<(usize, usize), u32> = HashMap::new();
        let mut previous_coords: Option<(usize, usize)> = None;
        for &(x, y) in input {
            draw_line(&mut tiles, previous_coords, x as usize, y as usize);
            previous_coords = Some((x as usize, y as usize));
        }
        let first_coord = input.first().unwrap();
        draw_line(
            &mut tiles,
            previous_coords,
            first_coord.0 as usize,
            first_coord.1 as usize,
        );

        for pair in input.iter().combinations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let size = (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1);

            if size <= biggest_valid_rectangle {
                continue;
            }
            let top_left_corner = (*x1.min(x2) as usize, *y1.min(y2) as usize);
            let bottom_right_corner = (*x1.max(x2) as usize, *y1.max(y2) as usize);

            if tiles.keys().any(|tile| {
                tile.0 < bottom_right_corner.0
                    && tile.0 > top_left_corner.0
                    && tile.1 > top_left_corner.1
                    && tile.1 < bottom_right_corner.1
            }) {
                continue;
            }
            biggest_valid_rectangle = size;
        }
        biggest_valid_rectangle
    }
}

pub fn draw_line(
    tiles: &mut HashMap<(usize, usize), u32>,
    previous_coords: Option<(usize, usize)>,
    x: usize,
    y: usize,
) {
    let x = x;
    let y = y;
    tiles.insert((x, y), 2);
    if let Some((prev_x, prev_y)) = previous_coords {
        if x == prev_x {
            let start = y.min(prev_y);
            let end = y.max(prev_y);
            for iter_y in (start + 1)..end {
                tiles.insert((x, iter_y), 1);
            }
        } else {
            let start = x.min(prev_x);
            let end = x.max(prev_x);
            for iter_x in (start + 1)..end {
                tiles.insert((iter_x, y), 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let parsed = Day09::parser(&mut INPUT).unwrap();
        assert_eq!(Day09::part_2(&parsed), 24);
    }
}
