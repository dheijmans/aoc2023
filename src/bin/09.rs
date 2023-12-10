advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let report = Report::parse(input);
    let predictions = report.predict();
    Some(predictions.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

struct Report {
    history: Vec<Vec<i64>>,
}

impl Report {
    fn parse(input: &str) -> Self {
        let history = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|value| value.parse().unwrap())
                    .collect()
            })
            .collect();
        Report { history }
    }

    fn predict(&self) -> Vec<i64> {
        self.history
            .iter()
            .map(|values| {
                let mut sequences: Vec<Vec<i64>> = vec![values.clone()];
                let mut i: usize = 0;
                while !sequences[i].iter().all(|value| value.eq(&0)) {
                    let mut sequence: Vec<i64> = Vec::new();
                    (0..(sequences[i].len() - 1)).for_each(|j| {
                        sequence.push(sequences[i][j + 1] - sequences[i][j]);
                    });
                    sequences.push(sequence);
                    i += 1;
                }
                sequences
                    .iter()
                    .rev()
                    .fold(0, |acc, sequence| sequence.last().unwrap() + acc)
            })
            .collect()
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
