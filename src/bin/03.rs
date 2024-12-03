use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            let a = a.parse::<u32>().expect("Expected number");
            let b = b.parse::<u32>().expect("Expected number");
            a * b
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    let mut should_execute = true;
    let mut result = 0;
    for caps in re.captures_iter(input) {
        let match_str = caps.get(0)?.as_str();
        if match_str == "don't()" {
            should_execute = false;
        } else if match_str == "do()" {
            should_execute = true;
        } else if should_execute {
            let (a, b) = (caps.get(1)?, caps.get(2)?);
            let a = a.as_str().parse::<u32>().expect("Expected number");
            let b = b.as_str().parse::<u32>().expect("Expected number");
            result += a * b;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
