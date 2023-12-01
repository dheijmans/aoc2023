advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut sum = 0;
    for line in &mut lines {
        let d1 = line.chars().find(|c| c.is_ascii_digit())?.to_digit(10)?;
        let d2 = line.chars().rfind(|c| c.is_ascii_digit())?.to_digit(10)?;
        sum += 10 * d1 + d2;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
