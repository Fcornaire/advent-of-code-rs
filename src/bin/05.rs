advent_of_code::solution!(5);
use std::{clone, fs};

use indicatif::ParallelProgressIterator;

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Map {
    pub destination: i64,
    pub source: i64,
    pub range: i64,
}

impl Map {
    pub fn new(data: Vec<i64>) -> Map {
        Map {
            destination: data[0],
            source: data[1],
            range: data[2],
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let seeds = get_seeds(input);

    let location: Vec<i64> = seeds
        .iter()
        .map(|seed| get_location(*seed, input))
        .collect();

    let res = *location.iter().min().unwrap();
    Some(res)
}

fn get_seeds(input: &str) -> Vec<i64> {
    get_split(input)
        .iter()
        .take(1)
        .flat_map(|s| {
            s.split_whitespace()
                .skip(1)
                .flat_map(|str| str.parse().ok())
        })
        .collect()
}

fn get_split(input: &str) -> Vec<&str> {
    input
        .split("\r\n\r\n")
        .flat_map(|str| str.split("\r\n"))
        .collect()
}

fn get_location(seed: i64, input: &str) -> i64 {
    let mut sp = get_split(input).into_iter().skip(1);
    let mut found = false;

    let mut curr = seed;

    while let Some(next) = sp.next() {
        if let Some(ch) = next.chars().next() {
            if !ch.is_digit(10) {
                found = false;
                continue;
            }
        }

        let numb = next
            .split_whitespace()
            .flat_map(|str| str.parse().ok())
            .collect::<Vec<i64>>();

        if !found && numb.len() > 0 {
            let map = Map::new(numb.clone());

            if map.source <= curr && curr <= map.range + map.source {
                curr = curr + map.destination - map.source;
                found = true;
            }
        }
    }

    curr
}

#[derive(Debug, Clone)]
enum Parsed {
    Seed(Vec<i64>),
    Map(Map),
    Nothing,
}

fn parse_input(input: &str) -> Vec<Parsed> {
    let mut sp = get_split(input).into_iter();
    let mut parsed = Vec::new();

    let fst = sp.next().unwrap();
    parsed.push(Parsed::Seed(
        fst.split_whitespace()
            .skip(1)
            .flat_map(|str| str.parse().ok())
            .collect(),
    ));

    while let Some(next) = sp.next() {
        if let Some(ch) = next.chars().next() {
            if !ch.is_digit(10) {
                parsed.push(Parsed::Nothing);
                continue;
            }
        }

        let numb = next
            .split_whitespace()
            .flat_map(|str| str.parse().ok())
            .collect::<Vec<i64>>();

        parsed.push(Parsed::Map(Map::new(numb)));
    }
    parsed
}

pub fn part_two(input: &str) -> Option<i64> {
    let parsed = parse_input(input);

    let fst_seed: Vec<i64> = parsed
        .clone()
        .into_par_iter()
        .filter_map(|data| match data {
            Parsed::Seed(seed) => Some(seed),
            _ => None,
        })
        .flatten()
        .collect::<Vec<i64>>()
        .into_par_iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, value)| value)
        .collect();

    let snd_seed: Vec<i64> = parsed
        .clone()
        .into_par_iter()
        .filter_map(|data| match data {
            Parsed::Seed(seed) => Some(seed),
            _ => None,
        })
        .flatten()
        .collect::<Vec<i64>>()
        .into_par_iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, value)| value)
        .collect();

    // println!("{:#?}", parsed);

    // let seeds: Vec<i64> = get_seeds(input);
    // let fst_seed: Vec<i64> = seeds
    //     .clone()
    //     .into_par_iter()
    //     .enumerate()
    //     .filter(|(index, _)| index % 2 == 0)
    //     .map(|(_, value)| value)
    //     .collect();

    // let snd_seed: Vec<i64> = seeds
    //     .clone()
    //     .into_par_iter()
    //     .enumerate()
    //     .filter(|(index, _)| index % 2 == 1)
    //     .map(|(_, value)| value)
    //     .collect();

    // let seed_range: Vec<(i64, i64)> = fst_seed
    //     .clone()
    //     .into_par_iter()
    //     .zip(snd_seed.into_par_iter())
    //     .collect();

    // let data: i64 = seed_range
    //     .into_par_iter()
    //     .flat_map_iter(|data| {
    //         let curr = data.0;
    //         let next = data.1;

    //         (curr..(next + curr))
    //             .into_par_iter()
    //             .progress_count(next as u64)
    //             .map(|data| get_location(data, input))
    //             .collect::<Vec<_>>()
    //     })
    //     .min()
    //     .unwrap();

    // Some(data)

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
