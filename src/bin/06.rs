advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time: Vec<f64> = parse_list(lines.next().unwrap(), "Time:");
    let distance: Vec<f64> = parse_list(lines.next().unwrap(), "Distance:");
    let races: Vec<Race> = time
        .iter()
        .zip(distance.iter())
        .map(|race| Race {
            time: *race.0,
            distance: *race.1,
        })
        .collect();
    let options: i64 = races
        .iter()
        .fold(1, |acc: i64, x| acc * x.calculate_options());
    Some(options as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time: f64 = parse_num(lines.next().unwrap(), "Time:");
    let distance: f64 = parse_num(lines.next().unwrap(), "Distance:");
    let race = Race { time, distance };
    let options: i64 = race.calculate_options();
    Some(options as u32)
}

fn parse_list(list: &str, prefix: &str) -> Vec<f64> {
    list.strip_prefix(prefix)
        .unwrap()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_num(num: &str, prefix: &str) -> f64 {
    num.strip_prefix(prefix)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn calculate_options(&self) -> i64 {
        let min = 0.5 * self.time - f64::sqrt(0.25 * self.time.powi(2) - self.distance);
        let max = 0.5 * self.time + f64::sqrt(0.25 * self.time.powi(2) - self.distance);
        max.ceil() as i64 - min.floor() as i64 - 1
    }
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
