use std::collections::HashMap;

use itertools::{Either, Itertools};

advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("failed to parse"))
                .collect::<Vec<_>>()
        })
        .enumerate()
        .partition_map(|(i, x)| match i % 2 {
            0 => Either::Left(x),
            _ => Either::Right(x),
        })
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left, right) = parse_input(input);
    let mut occurances: HashMap<i32, i32> = HashMap::new();

    for el in right {
        occurances
            .entry(el)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let result = left
        .iter()
        .map(|el| el * occurances.get(el).unwrap_or(&0i32))
        .sum::<i32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
