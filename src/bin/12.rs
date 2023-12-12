advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (mut conditions, sizes) = line.split_once(' ').map(|(a, b)| (String::from(a), b))?;
        conditions.push('.');
        let sizes: Vec<usize> = sizes.split(',').map(|x| x.parse().unwrap()).collect();
        let mut cache = vec![vec![None; sizes.len() + 1]; conditions.len() + 1];
        sum += count_arrangements(conditions.as_str(), &sizes, 0, &mut cache)?
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (conditions, sizes) = unfold(line)?;
        let mut cache = vec![vec![None; sizes.len() + 1]; conditions.len() + 1];
        sum += count_arrangements(conditions.as_str(), &sizes, 0, &mut cache)?
    }
    Some(sum)
}

fn unfold(input: &str) -> Option<(String, Vec<usize>)> {
    let split = input.split_once(' ')?;
    let original_conditions = split.0;
    let mut conditions: String = std::iter::repeat(original_conditions)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?");
    conditions.push('.');
    let original_sizes: Vec<usize> = split.1.split(',').map(|x| x.parse().unwrap()).collect();
    let mut sizes: Vec<usize> = Vec::new();
    for _ in 0..5 {
        sizes.extend(&original_sizes);
    }
    Some((conditions, sizes))
}

fn count_arrangements(
    conditions: &str,
    sizes: &Vec<usize>,
    index: usize,
    cache: &mut Vec<Vec<Option<u64>>>,
) -> Option<u64> {
    let (i, j) = (conditions.len(), index);
    if cache[i][j].is_some() {
        return cache[i][j];
    }

    if index >= sizes.len() {
        let result = match conditions.contains('#') {
            false => Some(1),
            true => Some(0),
        };
        cache[i][j] = result;
        return result;
    }

    let current = sizes[index];
    let mut arrangements: u64 = 0;

    if current > conditions.len() {
        let result = Some(0);
        cache[i][j] = result;
        return result;
    }

    for i in 0..=(conditions.len() - current) {
        if !(conditions[i..(i + current)].contains('.')
            || conditions.as_bytes()[i + current].eq(&b'#'))
        {
            arrangements +=
                count_arrangements(&conditions[(i + current + 1)..], sizes, index + 1, cache)?;
        }
        if conditions.as_bytes()[i].eq(&b'#') {
            break;
        }
    }
    let result = Some(arrangements);
    cache[i][j] = result;
    result
}
