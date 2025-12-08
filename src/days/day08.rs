use itertools::Itertools;
use std::collections::HashMap;
use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{opt, separated, terminated};
use winnow::{Parser, Result};

use crate::days::Day;

pub struct Day08;

#[derive(Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_line(input: &mut &str) -> Result<Position> {
    separated(1.., digit1.parse_to::<i32>(), ',')
        .map(|x: Vec<i32>| Position {
            x: x[0],
            y: x[1],
            z: x[2],
        })
        .parse_next(input)
}

impl Day for Day08 {
    type Input = Vec<Position>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        terminated(separated(1.., parse_line, line_ending), opt(line_ending)).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut distances: HashMap<(usize, usize), f64> = HashMap::new();
        generate_distances(input, &mut distances);
        let mut circuits: Vec<Vec<usize>> = vec![];

        join_first_1000(&mut distances, &mut circuits);
        circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .product()
    }

    type Output2 = i64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut distances: HashMap<(usize, usize), f64> = HashMap::new();
        generate_distances(input, &mut distances);
        let mut circuits: Vec<Vec<usize>> = vec![];
        let mut last_junction: (usize, usize) = (0, 0);

        join_first_1000(&mut distances, &mut circuits);

        let remaining_distances: Vec<((usize, usize), f64)> = distances
            .into_iter()
            .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .skip(1000)
            .collect();

        for (position_pair, _) in remaining_distances {
            let found1 = circuits
                .iter()
                .find_position(|circuit| circuit.contains(&position_pair.0));
            let found2 = circuits
                .iter()
                .find_position(|circuit| circuit.contains(&position_pair.1));
            if found1.is_some() && found2.is_some() {
                if found1.unwrap() == found2.unwrap() {
                    continue;
                } else {
                    let idx1 = found1.unwrap().0;
                    let idx2 = found2.unwrap().0;
                    let mut merged = found1.unwrap().1.clone();
                    merged.extend(found2.unwrap().1.iter().copied());

                    circuits[idx1] = merged;
                    circuits.remove(idx2);
                    last_junction = position_pair;
                }
            } else if found1.is_some() {
                let index = found1.unwrap().0;
                circuits[index].push(position_pair.1);
                last_junction = position_pair;
            } else if found2.is_some() {
                let index = found2.unwrap().0;
                circuits[index].push(position_pair.0);
                last_junction = position_pair;
            } else {
                circuits.push(vec![position_pair.0, position_pair.1]);
            }
        }
        input[last_junction.0].x as i64 * input[last_junction.1].x as i64
    }
}

fn join_first_1000(distances: &mut HashMap<(usize, usize), f64>, circuits: &mut Vec<Vec<usize>>) {
    for (position_pair, _) in distances
        .into_iter()
        .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .take(1000)
    {
        let found1 = circuits
            .iter()
            .find_position(|circuit| circuit.contains(&position_pair.0));
        let found2 = circuits
            .iter()
            .find_position(|circuit| circuit.contains(&position_pair.1));
        if found1.is_some() && found2.is_some() {
            if found1.unwrap() == found2.unwrap() {
                continue;
            } else {
                let idx1 = found1.unwrap().0;
                let idx2 = found2.unwrap().0;
                let mut merged = found1.unwrap().1.clone();
                merged.extend(found2.unwrap().1.iter().copied());

                circuits[idx1] = merged;
                circuits.remove(idx2);
            }
        } else if found1.is_some() {
            let index = found1.unwrap().0;
            circuits[index].push(position_pair.1);
        } else if found2.is_some() {
            let index = found2.unwrap().0;
            circuits[index].push(position_pair.0);
        } else {
            circuits.push(vec![position_pair.0, position_pair.1]);
        }
    }
}

fn generate_distances(input: &Vec<Position>, distances: &mut HashMap<(usize, usize), f64>) {
    for index_position in 0..input.len() {
        for index_other_position in index_position + 1..input.len() {
            distances.insert(
                (index_position, index_other_position),
                compute_distance(input[index_position], input[index_other_position]),
            );
        }
    }
}

fn compute_distance(p1: Position, p2: Position) -> f64 {
    let dx: i128 = (p2.x - p1.x) as i128;
    let dy: i128 = (p2.y - p1.y) as i128;
    let dz: i128 = (p2.z - p1.z) as i128;

    ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
}
