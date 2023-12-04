use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let res: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let numbers = line.split(":").collect::<Vec<&str>>()[1];
            let splited: Vec<&str> = numbers.split("|").collect();

            let winning: HashSet<u32> = splited[0]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let owning: HashSet<u32> = splited[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let common_numbers: Vec<u32> = winning.intersection(&owning).cloned().collect();

            let mut point = 0;

            for _ in common_numbers {
                if point == 0 {
                    point = 1;
                } else {
                    point *= 2;
                }
            }

            point
        })
        .collect();

    Some(res.iter().sum())
}

//Solution from part_2 came from https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day4/src/main.rs

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_counts = vec![1u32];
    let mut count = 0;
    let mut winning = Vec::new();

    for (n, line) in input.lines().enumerate() {
        let end = n + get_matches(line, &mut winning) + 1;

        if end > card_counts.len() {
            card_counts.resize(end, 1);
        }

        for i in n + 1..end {
            card_counts[i] += card_counts[n];
        }

        count += card_counts[n];
    }

    Some(count)
}

fn get_matches<'a>(line: &'a str, winning: &mut Vec<&'a str>) -> usize {
    let mut words = line.split_whitespace().skip(2);
    winning.clear();
    while let Some(x) = words.next() {
        if x == "|" {
            break;
        }

        winning.push(x);
    }

    words.filter(|x| winning.contains(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
