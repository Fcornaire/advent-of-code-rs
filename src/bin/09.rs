use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

advent_of_code::solution!(9);

#[derive(Debug)]
struct History {
    pub numbers: Vec<i32>,
}

impl History {
    pub fn get_next_extrapolated_value(&self) -> i32 {
        let mut sequences = self
            .produce_sequences()
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<Vec<i32>>>();

        sequences.first_mut().unwrap().push(0);

        let res = sequences
            .iter()
            .enumerate()
            .fold(0, |acc, (index, sequence)| {
                if index == 0 {
                    return acc;
                }

                let last = sequence.last().unwrap();

                acc + last
            });

        res
    }

    pub fn get_next_extrapolated_value_backward(&self) -> i32 {
        let mut sequences = self
            .produce_sequences()
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<Vec<i32>>>();

        sequences.first_mut().unwrap().push(0);

        let res = sequences
            .iter()
            .enumerate()
            .fold(0, |acc, (index, sequence)| {
                if index == 0 {
                    return acc;
                }

                let first = sequence.first().unwrap();

                first - acc
            });

        res
    }

    fn produce_sequences(&self) -> Vec<Vec<i32>> {
        let mut sequence = Vec::new();
        let mut copy = self.numbers.clone();

        sequence.push(copy.clone());

        while !copy.par_iter().all(|number| *number == 0) {
            let clone = copy.clone();
            copy.par_iter_mut().enumerate().for_each(|(index, number)| {
                if index == clone.len() - 1 {
                    return;
                }

                *number = clone[index + 1] - *number;
            });

            copy.remove(copy.len() - 1);

            sequence.push(copy.clone());
        }

        sequence
    }
}

type Report = Vec<History>;

fn parse_input(input: &str) -> Report {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            History { numbers }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let report: Report = parse_input(input);

    let res: i32 = report
        .par_iter()
        .map(|history| history.get_next_extrapolated_value())
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let report: Report = parse_input(input);

    let res: i32 = report
        .par_iter()
        .map(|history| history.get_next_extrapolated_value_backward())
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
