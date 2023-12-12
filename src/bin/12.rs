advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (mut conditions, sizes) = line.split_once(' ').map(|(a, b)| (String::from(a), b))?;
        conditions.push('.');
        let sizes: Vec<usize> = sizes.split(',').map(|x| x.parse().unwrap()).collect();
        sum += count_arrangements(conditions.as_str(), &sizes, 0)?
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn count_arrangements(conditions: &str, sizes: &Vec<usize>, index: usize) -> Option<u32> {
    if index >= sizes.len() {
        return Some(1);
    }

    let current = sizes[index];
    let mut arrangements: u32 = 0;

    if current > conditions.len() {
        return Some(0);
    }

    for i in 0..=(conditions.len() - current) {
        if (conditions[i..(i + current)].contains('.'))
            || (i > 0 && conditions.chars().nth(i - 1)?.eq(&'#'))
            || (i + current < conditions.len() && conditions.chars().nth(i + current)?.eq(&'#'))
        {
            continue;
        }
        arrangements += count_arrangements(&conditions[(i + current + 1)..], sizes, index + 1)?;
    }
    Some(arrangements)
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
