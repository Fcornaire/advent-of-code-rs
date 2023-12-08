use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use indicatif::{ParallelProgressIterator, ProgressIterator};
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    current: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (Vec<Instruction>, Vec<Node>) {
    let instructions: Vec<Instruction> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            })
        })
        .collect();

    let nodes: Vec<Node> = input
        .lines()
        .skip(2)
        .map(|line| {
            let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
            let caps = re.captures(line).unwrap();

            let current = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let left = caps.get(2).map_or("", |m| m.as_str()).to_string();
            let right = caps.get(3).map_or("", |m| m.as_str()).to_string();

            Node {
                current,
                left,
                right,
            }
        })
        .collect();

    (instructions, nodes)
}

static SAVED_NODES: Lazy<Arc<Mutex<HashMap<String, Node>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn find_node_and_save(nodes: Vec<Node>, name: &str) -> Option<Node> {
    if let Some(node) = SAVED_NODES.clone().lock().unwrap().get(name) {
        return Some(node.clone());
    }

    let node = nodes
        .par_iter()
        .progress()
        .find_first(|node| node.current == name)
        .cloned();

    if let Some(node) = node.clone() {
        SAVED_NODES
            .clone()
            .lock()
            .unwrap()
            .insert(name.to_string(), node);
    }

    node
}

fn find_nodes_ending_with(nodes: Vec<Node>, ending: char) -> HashSet<Node> {
    nodes
        .par_iter()
        .filter(|node| node.current.ends_with(ending))
        .map(|node| node.clone())
        .collect()
}

fn is_node_ending_with(node: Node, ending: char) -> bool {
    node.current.ends_with(ending)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, nodes) = parse_input(input);

    let mut current = find_node_and_save(nodes.clone(), "AAA").unwrap();

    let mut index = 0;
    let mut count = 0;

    while current.current != "ZZZ" {
        let instruction = instructions.get(index).unwrap();

        count += 1;

        match instruction {
            Instruction::Left => {
                current = find_node_and_save(nodes.clone(), &current.left).unwrap();
            }
            Instruction::Right => {
                current = find_node_and_save(nodes.clone(), &current.right).unwrap();
            }
        }

        if instructions.get(index + 1).is_none() {
            index = 0;
        } else {
            index += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse_input(input);
    let currents = find_nodes_ending_with(nodes.clone(), 'A');

    let path_to_z: Vec<usize> = currents
        .iter()
        .cloned()
        .collect::<Vec<Node>>()
        .par_iter()
        .progress()
        .map(|node| {
            let mut iteration = 0;
            let mut index = 0;
            let mut node = node.clone();

            while !is_node_ending_with(node.clone(), 'Z') {
                let instruction = instructions.get(index).unwrap();

                iteration += 1;
                match instruction {
                    Instruction::Left => {
                        node = find_node_and_save(nodes.clone(), &node.left).unwrap();
                    }
                    Instruction::Right => {
                        node = find_node_and_save(nodes.clone(), &node.right).unwrap();
                    }
                }

                if instructions.get(index + 1).is_none() {
                    index = 0;
                } else {
                    index += 1;
                }
            }

            iteration
        })
        .collect();

    let lcm = path_to_z
        .into_iter()
        .progress()
        .fold(1, |acc, x| num::integer::lcm(acc, x));

    Some(lcm.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
