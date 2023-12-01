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
    let mut lines = input.lines();
    let mut sum = 0;
    let decimals: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in &mut lines {
        let mut digits: Vec<u32> = Vec::new();
        let bytes = line.as_bytes();
        for (i, &char) in bytes.iter().enumerate() {
            if char.is_ascii_digit() {
                digits.push((char - b'0') as u32)
            } else {
                (0..decimals.len()).for_each(|j| {
                    if line[i..].starts_with(decimals[j]) {
                        digits.push(j as u32 + 1)
                    }
                });
            }
        }
        sum += digits.first()? * 10 + digits.last()?
    }
    Some(sum)
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
