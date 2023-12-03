use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let (mut sum, mut game) = (0, 1);
    let regex = Regex::new(r"; |, ").unwrap();
    for line in lines {
        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
        let cubes: Vec<&str> = regex.split(line.split_once(": ")?.1).collect();
        for cube_type in cubes {
            let (amount, color) = cube_type.split_once(' ')?;
            let amount = amount.parse().unwrap();
            match color {
                "red" => r = r.max(amount),
                "green" => g = g.max(amount),
                "blue" => b = b.max(amount),
                _ => panic!(),
            }
        }
        if r <= 12 && g <= 13 && b <= 14 {
            sum += game;
        }
        game += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    let regex = Regex::new(r"; |, ").unwrap();
    for line in lines {
        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
        let cubes: Vec<&str> = regex.split(line.split_once(": ")?.1).collect();
        for cube_type in cubes {
            let (amount, color) = cube_type.split_once(' ')?;
            let amount = amount.parse().unwrap();
            match color {
                "red" => r = r.max(amount),
                "green" => g = g.max(amount),
                "blue" => b = b.max(amount),
                _ => panic!(),
            }
        }
        sum += r * g * b;
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
