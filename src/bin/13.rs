advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    summarize_notes(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    summarize_notes(input, true)
}

fn summarize_notes(input: &str, smudge: bool) -> Option<u32> {
    let patterns = input.split("\n\n");
    let mut sum = 0;
    for pattern in patterns {
        let (rows, columns) = parse(pattern)?;
        sum += write_note(rows, columns, smudge)?;
    }
    Some(sum)
}

fn parse(pattern: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let height = pattern.lines().count();
    let width = pattern.lines().next()?.len();
    let mut rows: Vec<u32> = vec![0; height];
    let mut columns: Vec<u32> = vec![0; width];
    for (i, line) in pattern.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            rows[i] <<= 1;
            columns[j] <<= 1;
            if char.eq(&'#') {
                rows[i] += 1;
                columns[j] += 1;
            }
        }
    }
    Some((rows, columns))
}

fn write_note(rows: Vec<u32>, columns: Vec<u32>, smudge: bool) -> Option<u32> {
    let horizontal_lines = find_mirror(rows, smudge);
    match horizontal_lines.is_some() {
        true => Some(100 * horizontal_lines?),
        false => find_mirror(columns, smudge),
    }
}

fn find_mirror(list: Vec<u32>, smudge: bool) -> Option<u32> {
    let mirrors: Vec<usize> = list
        .windows(2)
        .enumerate()
        .filter(|&(_, w)| match smudge {
            true => hamming_distance(w[0], w[1]) <= 1,
            false => w[0] == w[1],
        })
        .map(|(index, _)| index + 1)
        .collect();
    for mirror in mirrors {
        let range = mirror.min(list.len() - mirror);
        if validate_mirror(&list[(mirror - range)..(mirror + range)], smudge as u8) {
            return Some(mirror as u32);
        }
    }
    None
}

fn validate_mirror(list: &[u32], mutations: u8) -> bool {
    let mut smudges = 0;
    list.iter()
        .zip(list.iter().rev())
        .take(list.len() / 2)
        .all(|(&a, &b)| match hamming_distance(a, b) {
            0 => true,
            1 => {
                smudges += 1;
                true
            }
            _ => false,
        })
        && smudges == mutations
}

fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}
