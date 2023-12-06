advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn get_all_wr_distances(&self) -> Vec<u64> {
        let mut distances: Vec<u64> = vec![];

        for time in 0..self.time {
            distances.push(self.get_distance_at_time(time));
        }

        distances.retain(|&distance| distance > self.distance);

        distances
    }

    pub fn get_all_wr(&self) -> usize {
        let wr = self.get_all_wr_distances();

        wr.iter().count()
    }

    fn get_distance_at_time(&self, time: u64) -> u64 {
        if time == 0 || time == self.time {
            return 0;
        }

        let speed = time;

        speed * (self.time - time)
    }
}

fn get_times(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .into_iter()
        .take(1)
        .flat_map(|str| str.split_whitespace())
        .skip(1)
        .flat_map(|num| num.parse().ok())
        .collect::<Vec<u64>>()
}

fn get_solo_times(input: &str) -> u64 {
    let data = input
        .split("\n")
        .into_iter()
        .take(1)
        .flat_map(|str| str.split_whitespace())
        .skip(1)
        .collect::<Vec<&str>>();

    let mut result = String::new(); // String to store the result

    for num in data {
        result.push_str(&num.to_string());
    }

    result.parse().unwrap()
}

fn get_solo_distance(input: &str) -> u64 {
    let data = input
        .split("\n")
        .into_iter()
        .skip(1)
        .flat_map(|str| str.split_whitespace())
        .skip(1)
        .collect::<Vec<&str>>();

    let mut result = String::new(); // String to store the result

    for num in data {
        result.push_str(&num.to_string());
    }

    println!("{}", result);

    result.parse().unwrap()
}

fn get_distance(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .into_iter()
        .skip(1)
        .flat_map(|str| str.split_whitespace())
        .skip(1)
        .flat_map(|num| num.parse().ok())
        .collect::<Vec<u64>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let times = get_times(input);
    let distance = get_distance(input);
    let mut races: Vec<Race> = vec![];

    for i in 0..times.len() {
        races.push(Race::new(times[i], distance[i]));
    }

    let records: Vec<usize> = races.iter().map(|race| race.get_all_wr()).collect();

    let res = records.iter().fold(1, |acc, x| acc * x);

    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let time = get_solo_times(input);
    let distance = get_solo_distance(input);
    let race = Race::new(time, distance);
    let record = race.get_all_wr();

    Some(record as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
