use std::{
    collections::{HashMap, HashSet},
    str::Split,
};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (workflow_input, parts_input) = input.split_once("\n\n")?;
    let workflows = parse_workflows(workflow_input)?;
    let parts = parse_parts(parts_input)?;
    let mut sum = 0;
    for part in parts {
        if part.evaluate(&workflows)? {
            sum += part.rate();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflow_input, _) = input.split_once("\n\n")?;
    let workflows = parse_workflows(workflow_input)?;
    let ranges = calculate_accepted_ranges(&workflows)?;
    let combinations = ranges.iter().fold(0, |acc, range| acc + range.combos());
    Some(combinations)
}

fn parse_workflows(input: &str) -> Option<HashMap<String, Workflow>> {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for line in input.lines() {
        let (id, workflow) = Workflow::parse(line)?;
        workflows.insert(id, workflow);
    }
    Some(workflows)
}

fn parse_parts(input: &str) -> Option<Vec<Part>> {
    let mut parts: Vec<Part> = Vec::new();
    for line in input.lines() {
        let part = Part::parse(line)?;
        parts.push(part);
    }
    Some(parts)
}

fn calculate_accepted_ranges(workflows: &HashMap<String, Workflow>) -> Option<Vec<PartRange>> {
    let mut accepted: Vec<PartRange> = Vec::new();
    let mut current: Vec<(PartRange, &Workflow)> =
        vec![(PartRange::default(), workflows.get("in")?)];
    while !current.is_empty() {
        let mut next: Vec<(PartRange, &Workflow)> = Vec::new();
        'outer: for (mut range, workflow) in current {
            for rule in &workflow.rules {
                let (condition, result) = rule.split_once(':')?;
                let category = condition.chars().next()?;
                let threshold: u32 = condition[2..].parse().ok()?;
                let operation = condition.chars().nth(1)?;
                let (inside, outside);
                match operation {
                    '<' => {
                        (inside, outside) = range.split(category, threshold);
                    }
                    '>' => {
                        (outside, inside) = range.split(category, threshold + 1);
                    }
                    _ => panic!(),
                }
                if inside.is_some() {
                    match result {
                        "A" => accepted.push(inside?),
                        "R" => (),
                        next_workflow => next.push((inside?, workflows.get(next_workflow)?)),
                    }
                }
                if let Some(partition) = outside {
                    range = partition;
                } else {
                    continue 'outer;
                }
            }
            match workflow.default.as_str() {
                "A" => accepted.push(range),
                "R" => (),
                next_workflow => next.push((range, workflows.get(next_workflow)?)),
            }
        }
        current = next;
    }
    Some(accepted)
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(input: &str) -> Option<Self> {
        let mut categories: Split<'_, char> =
            input.strip_prefix('{')?.strip_suffix('}')?.split(',');
        fn pop_rating(categories: &mut Split<'_, char>) -> Option<u32> {
            categories.next()?.split_once('=')?.1.parse().ok()
        }
        let x: u32 = pop_rating(&mut categories)?;
        let m: u32 = pop_rating(&mut categories)?;
        let a: u32 = pop_rating(&mut categories)?;
        let s: u32 = pop_rating(&mut categories)?;
        Some(Self { x, m, a, s })
    }

    fn evaluate(&self, workflows: &HashMap<String, Workflow>) -> Option<bool> {
        let mut current = workflows.get("in")?;
        let mut evaluated: HashSet<String> = HashSet::new();
        evaluated.insert("in".to_owned());
        loop {
            match current.evaluate(self)?.as_str() {
                "A" => return Some(true),
                "R" => return Some(false),
                next => {
                    if evaluated.contains(next) {
                        return None;
                    }
                    evaluated.insert(next.to_owned());
                    current = workflows.get(next)?;
                }
            }
        }
    }

    fn rate(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn combos(&self) -> u64 {
        (self.end - self.start).into()
    }

    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: 0,
            end: 4000,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn split(&self, category: char, threshold: u32) -> (Option<Self>, Option<Self>) {
        let range = match category {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!(),
        };
        if threshold <= range.start {
            return (None, Some(*self));
        }
        if threshold <= range.start {
            return (Some(*self), None);
        }
        (
            Some(self.mutate(category, Range::new(range.start, threshold - 1))),
            Some(self.mutate(category, Range::new(threshold, range.end))),
        )
    }

    fn mutate(&self, category: char, range: Range) -> Self {
        let mut mutation: PartRange = *self;
        match category {
            'x' => mutation.x = range,
            'm' => mutation.m = range,
            'a' => mutation.a = range,
            's' => mutation.s = range,
            _ => panic!(),
        };
        mutation
    }

    fn combos(&self) -> u64 {
        self.x.combos() * self.m.combos() * self.a.combos() * self.s.combos()
    }
}

struct Workflow {
    rules: Vec<String>,
    default: String,
}

impl Workflow {
    fn parse(input: &str) -> Option<(String, Self)> {
        let (id, rules_input) = input.split_once('{')?;
        let mut rules: Vec<String> = rules_input
            .strip_suffix('}')?
            .split(',')
            .map(|rule| rule.to_owned())
            .collect();
        let default: String = rules.pop()?;
        Some((id.to_owned(), Self { rules, default }))
    }

    fn evaluate(&self, part: &Part) -> Option<String> {
        for rule in &self.rules {
            let (condition, result) = rule.split_once(':')?;
            let category = condition.chars().nth(0)?;
            let value = match category {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!(),
            };
            let threshold: u32 = condition[2..].parse().ok()?;
            let operation = condition.chars().nth(1)?;
            match operation {
                '>' if value > threshold => return Some(result.to_string()),
                '<' if value < threshold => return Some(result.to_string()),
                _ => continue,
            }
        }
        Some(self.default.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
