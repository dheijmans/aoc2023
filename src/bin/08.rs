use num::Integer;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (chars, nodes) = input.split_once("\n\n")?;
    let instructions: Vec<char> = chars.chars().collect();
    let mut network: HashMap<String, Node> = HashMap::new();
    nodes.lines().for_each(|line| {
        let id = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        network.insert(id.clone(), Node::new(id, left, right));
    });
    calculate_steps(instructions, network)
}

fn calculate_steps(instructions: Vec<char>, network: HashMap<String, Node>) -> Option<u32> {
    let mut ptr = 0;
    let mut current = network.get(&String::from("AAA"))?;
    let mut steps: u32 = 0;
    loop {
        if current.id == "ZZZ" {
            break;
        }
        current = match instructions[ptr] {
            'L' => network.get(&current.left)?,
            'R' => network.get(&current.right)?,
            _ => panic!(),
        };
        steps += 1;
        ptr = (ptr + 1) % instructions.len();
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (chars, nodes) = input.split_once("\n\n")?;
    let instructions: Vec<char> = chars.chars().collect();
    let mut network: HashMap<String, Node> = HashMap::new();
    nodes.lines().for_each(|line| {
        let id = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        network.insert(id.clone(), Node::new(id, left, right));
    });
    calculate_ghost_steps(instructions, network)
}

fn calculate_ghost_steps(instructions: Vec<char>, network: HashMap<String, Node>) -> Option<u64> {
    let start_nodes: Vec<&Node> = network
        .iter()
        .map(|entry| entry.1)
        .filter(|&node| node.id.ends_with('A'))
        .collect();
    let total_steps = start_nodes.iter().fold(1, |acc, &x| {
        let mut ptr = 0;
        let mut current = x;
        let mut steps = 0;
        loop {
            if current.id.ends_with('Z') {
                break;
            }
            current = match instructions[ptr] {
                'L' => network.get(&current.left).expect("Left Node"),
                'R' => network.get(&current.right).expect("Right Node"),
                _ => panic!(),
            };
            steps += 1;
            ptr = (ptr + 1) % instructions.len();
        }
        acc.lcm(&steps)
    });
    Some(total_steps)
}

struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn new(id: String, left: String, right: String) -> Self {
        Node { id, left, right }
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
