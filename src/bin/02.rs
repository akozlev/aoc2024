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

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().flat_map(|line| {
        let nums = line
            .split(' ')
            .map(|n| n.parse::<u32>().expect("Unexpected non-number input"));

        for num in nums {}

        let row = line
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| {
                let diff = a.abs_diff(b);
                ((1..=3).contains(&diff), a.cmp(&b))
            })
            .collect::<Vec<_>>();
        println!("{row:?}");
        row
    });

    Some(lines.count())
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
