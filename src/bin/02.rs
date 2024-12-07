use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .flat_map(|line| {
            line.split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| {
                    let diff = a.abs_diff(b);
                    if !(1..=3).contains(&diff) {
                        return None;
                    }

                    Some(a.cmp(&b))
                })
                .collect::<Option<Vec<_>>>()
        })
        .filter(|row| row.iter().all_equal());

    Some(lines.count())
}

pub fn is_valid(sequence: Vec<&u32>) -> bool {
    let iter = sequence.iter().tuple_windows::<(_, _)>().map(|(a, b)| {
        let diff = a.abs_diff(**b);
        if !(1..=3).contains(&diff) {
            return None;
        }

        Some(a.cmp(b))
    });

    iter.clone().all(|x| x.is_some()) && iter.clone().all_equal()
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().map(|line| {
        let nums = line
            .split(' ')
            .map(|n| n.parse::<u32>().expect("Unexpected non-number input"))
            .collect::<Vec<_>>();

        let any_valid = nums
            .iter()
            .combinations(nums.len() - 1)
            .any(|seq| is_valid(seq));

        any_valid
    });

    Some(lines.filter(|&x| x).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
