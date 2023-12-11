advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut galaxies = parse(input);

    galaxies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let x_distance: u64 = calculate_distance(&galaxies, true, 2);

    galaxies.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let y_distance: u64 = calculate_distance(&galaxies, false, 2);

    Some(x_distance + y_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut galaxies = parse(input);

    galaxies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let x_distance: u64 = calculate_distance(&galaxies, true, 1000000);

    galaxies.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let y_distance: u64 = calculate_distance(&galaxies, false, 1000000);

    Some(x_distance + y_distance)
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.eq(&'#') {
                galaxies.push((x, y))
            }
        }
    }
    galaxies
}

fn calculate_distance(galaxies: &Vec<(usize, usize)>, horizontal: bool, expension: usize) -> u64 {
    galaxies
        .iter()
        .map(|&(x, y)| if horizontal { x } else { y })
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|w| w[1] - w[0])
        .map(|d| if d > 1 { (d - 1) * expension + 1 } else { d })
        .enumerate()
        .map(|(i, d)| d * (i + 1) * (galaxies.len() - 1 - i))
        .sum::<usize>() as u64
}
