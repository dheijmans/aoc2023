advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sequence = input.lines().next()?;
    let mut sum = 0;
    for step in sequence.split(',') {
        sum += hash(step);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sequence = input.lines().next()?;
    let mut boxes: Vec<Box> = vec![Box::default(); 256];
    for step in sequence.split(',') {
        match step.chars().last() {
            Some('-') => remove(&mut boxes, step),
            _ => add(&mut boxes, step),
        }
    }
    let sum = boxes
        .iter()
        .zip(1..)
        .fold(0, |acc, (b, i)| acc + i * b.score());
    Some(sum)
}

fn add(boxes: &mut [Box], op: &str) {
    let mut splits = op.split('=');
    let label = splits.next().unwrap();
    let focal_length = splits.next().unwrap().parse::<u32>().unwrap();
    let hash = hash(label) as usize;
    let lens = boxes[hash]
        .lenses
        .iter_mut()
        .find(|lens| lens.label == label);
    match lens {
        Some(lens) => lens.focal_length = focal_length,
        None => boxes[hash]
            .lenses
            .push(Lens::new(label.to_string(), focal_length)),
    }
}

fn remove(boxes: &mut [Box], op: &str) {
    let label = op.split_once('-').unwrap().0;
    let hash = hash(label) as usize;
    boxes[hash].lenses.retain(|lens| lens.label != label);
}

fn hash(step: &str) -> u32 {
    let mut current_value = 0;
    for char in step.chars() {
        current_value += char as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[derive(Default, Clone)]
struct Box {
    lenses: Vec<Lens>,
}
impl Box {
    fn score(&self) -> u32 {
        self.lenses
            .iter()
            .zip(1..)
            .fold(0, |acc, (lens, i)| acc + i * lens.focal_length)
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Lens {
    fn new(label: String, focal_length: u32) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}
