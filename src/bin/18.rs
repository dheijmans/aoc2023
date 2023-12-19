use nalgebra::{point, vector, Point2, Vector2};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    calculate_area(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    calculate_area(input, true)
}

fn calculate_area(input: &str, use_color: bool) -> Option<u64> {
    let mut current_trench: Point2<i64> = point![0, 0];
    let mut area: i64 = 0;
    let mut trenches: i64 = 0;
    for line in input.lines() {
        let direction: Vector2<i64> = calculate_direction(line, use_color)?;
        let distance: i64 = calculate_distance(line, use_color)?;
        let new_trench: Point2<i64> = current_trench + distance * direction;
        area += (current_trench.y + new_trench.y) * (current_trench.x - new_trench.x);
        trenches += distance;
        current_trench = new_trench;
    }
    Some(((area.abs() + trenches) / 2 + 1) as u64)
}

fn calculate_direction(line: &str, use_color: bool) -> Option<Vector2<i64>> {
    let mut splits = line.split(' ');
    let instruction = splits.next()?;
    let _ = splits.next();
    let color = splits.next()?.strip_prefix("(#")?.strip_suffix(')')?;
    match use_color {
        true => match color.chars().last()? {
            '0' => Some(vector![1, 0]),
            '1' => Some(vector![0, -1]),
            '2' => Some(vector![-1, 0]),
            '3' => Some(vector![0, 1]),
            _ => panic!(),
        },
        false => match instruction {
            "U" => Some(vector![0, 1]),
            "D" => Some(vector![0, -1]),
            "L" => Some(vector![-1, 0]),
            "R" => Some(vector![1, 0]),
            _ => panic!(),
        },
    }
}

fn calculate_distance(line: &str, use_color: bool) -> Option<i64> {
    let mut splits = line.split(' ');
    let _ = splits.next();
    let instruction = splits.next()?;
    let color = splits.next()?.strip_prefix("(#")?.strip_suffix(')')?;
    if !use_color {
        instruction.parse().ok()
    } else {
        i64::from_str_radix(&color[..5], 16).ok()
    }
}
