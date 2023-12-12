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
        match conditions.contains('#') {
            false => return Some(1),
            true => return Some(0),
        }
    }

    let current = sizes[index];
    let mut arrangements: u32 = 0;

    if current > conditions.len() {
        return Some(0);
    }

    for i in 0..=(conditions.len() - current) {
        if !((conditions[i..(i + current)].contains('.'))
            || (i > 0 && conditions[..i].contains('#'))
            || (i + current < conditions.len() && conditions.chars().nth(i + current)?.eq(&'#')))
        {
            arrangements += count_arrangements(&conditions[(i + current + 1)..], sizes, index + 1)?;
        }
    }
    Some(arrangements)
}
