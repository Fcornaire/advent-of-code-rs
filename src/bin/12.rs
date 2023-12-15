use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::iter::once;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operationnal,
    Damaged,
    Unknown,
}

impl Spring {
    pub fn is_unknown(&self) -> bool {
        match self {
            Spring::Unknown => true,
            _ => false,
        }
    }

    pub fn is_operationnal(&self) -> bool {
        match self {
            Spring::Operationnal => true,
            _ => false,
        }
    }

    pub fn is_damaged(&self) -> bool {
        match self {
            Spring::Damaged => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Record {
    pub condition: Vec<Spring>,
    pub damaged: Vec<usize>,
}

impl Record {
    pub fn new(condition: Vec<Spring>, damaged: Vec<usize>) -> Self {
        Self { condition, damaged }
    }

    fn _get_condition_combination(&self) -> (Vec<usize>, Vec<Vec<Spring>>) {
        let unknows = self
            .condition
            .par_iter()
            .enumerate()
            .filter(|(_, spring)| spring.is_unknown())
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let result = Arc::new(Mutex::new(vec![]));
        let cond = Mutex::new(self.condition.clone());

        (0..2usize.pow(unknows.len() as u32))
            .into_par_iter()
            .for_each(|i| {
                let mut combination = cond.lock().unwrap().clone();
                let mut j = 0;
                for k in unknows.iter() {
                    combination[*k] = if i & (1 << j) != 0 {
                        Spring::Operationnal
                    } else {
                        Spring::Damaged
                    };
                    j += 1;
                }
                result.clone().lock().unwrap().push(combination);
            });

        (
            self.damaged.clone(),
            result.clone().lock().unwrap().to_vec(),
        )
    }
}

fn count(condition: Vec<Spring>, damaged: Vec<usize>) -> usize {
    if condition.is_empty() {
        if damaged.is_empty() {
            return 1;
        }
        return 0;
    }

    if damaged.is_empty() {
        if condition.iter().any(|s| s.is_damaged()) {
            return 0;
        }
        return 1;
    }

    let key = (condition.clone(), damaged.clone());

    if let Some(cached) = SAVED_RECORDS.lock().unwrap().get(&key) {
        return *cached;
    }

    let mut result = 0;

    if condition[0].is_operationnal() || condition[0].is_unknown() {
        let mut cond = condition.clone();
        cond.remove(0);

        result += count(cond, damaged.clone());
    }

    if condition[0].is_damaged() || condition[0].is_unknown() {
        if damaged[0] <= condition.len()
            && condition
                .iter()
                .take(damaged[0])
                .all(|spring| !spring.is_operationnal())
            && (damaged[0] == condition.len() || !condition[damaged[0]].is_damaged())
        {
            let cond = condition
                .iter()
                .skip(damaged[0] + 1)
                .cloned()
                .collect::<Vec<Spring>>();

            let mut dmg = damaged.clone();
            dmg.remove(0);

            result += count(cond, dmg);
        }
    }

    SAVED_RECORDS.lock().unwrap().insert(key, result);

    result
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .par_lines()
        .map(|line| {
            let (condition, damaged) = line
                .split_whitespace()
                .next_tuple()
                .map(|(s1, s2)| {
                    (
                        s1.chars()
                            .map(|c| match c {
                                '.' => Spring::Operationnal,
                                '#' => Spring::Damaged,
                                _ => Spring::Unknown,
                            })
                            .collect::<Vec<Spring>>(),
                        s2.split(",")
                            .flat_map(|s| s.parse::<usize>())
                            .collect::<Vec<usize>>(),
                    )
                })
                .unwrap_or_default();

            Record::new(condition, damaged)
        })
        .collect()
}

//Old version
fn _get_all_valid_combinations(records: Vec<Record>) -> Vec<Vec<Spring>> {
    let result = Arc::new(Mutex::new(vec![]));

    records
        .par_iter()
        .progress()
        .map(|r| r._get_condition_combination())
        .for_each(|(damaged, combinations)| {
            combinations
                .par_iter()
                .filter(|c| _is_valid_combination(c.to_vec(), damaged.clone()))
                .for_each(|c| result.clone().lock().unwrap().push(c.clone()));
        });

    result.clone().lock().unwrap().clone()
}

fn _is_valid_combination(condition: Vec<Spring>, damaged: Vec<usize>) -> bool {
    let mut current_damaged_ind = 0;
    let mut is_counting_damaged = false;
    let mut counted_damaged = 0;
    let mut current = 0;

    let mut ite = condition.iter();

    while let Some(spring) = ite.next() {
        current += 1;

        match spring {
            Spring::Damaged => {
                if !is_counting_damaged {
                    is_counting_damaged = true;
                }

                counted_damaged += 1;
            }
            Spring::Operationnal => {
                if is_counting_damaged {
                    is_counting_damaged = false;

                    if current_damaged_ind < damaged.len() {
                        if counted_damaged != damaged[current_damaged_ind] {
                            return false;
                        }
                    }

                    if current == condition.len() {
                        break;
                    }

                    current_damaged_ind += 1;

                    counted_damaged = 0;
                }
            }
            _ => {}
        }
    }

    if let Some(lst) = condition.clone().last() {
        if let Some(pre_last) = condition.clone().get(condition.len() - 2) {
            if lst.is_operationnal() && !pre_last.is_damaged() {
                current_damaged_ind -= 1;
            }

            if lst.is_damaged() {
                if current_damaged_ind < damaged.len() {
                    if counted_damaged != damaged[current_damaged_ind] {
                        return false;
                    }
                }
            }
        }
    }

    if current_damaged_ind == damaged.len() - 1 && current == condition.len() {
        debug!("is_valid [{:?}] : {:?} ", damaged, condition);
    }

    current_damaged_ind == damaged.len() - 1 && current == condition.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let records = parse_input(input);

    let res = records
        .par_iter()
        .progress()
        .map(|rec| count(rec.condition.clone(), rec.damaged.clone()))
        .collect::<Vec<usize>>();

    debug!("Count: {:?}", res);

    Some(res.iter().sum::<usize>() as u32)
}

fn extend_records(records: &mut Vec<Record>, extender: usize) {
    records.iter_mut().for_each(|record| {
        let ori = record.condition.clone();

        let mut result_vec: Vec<_> = (0..extender - 1)
            .into_par_iter()
            .flat_map(|_| ori.clone().into_par_iter().chain(once(Spring::Unknown)))
            .collect();

        result_vec.extend_from_slice(&ori);

        record.condition = result_vec;
    });

    records.iter_mut().for_each(|record| {
        let ori = record.damaged.clone();

        let result_vec: Vec<_> = (0..extender)
            .into_par_iter()
            .flat_map(|_| ori.clone())
            .collect();

        record.damaged = result_vec;
    });
}

static SAVED_RECORDS: Lazy<Arc<Mutex<HashMap<(Vec<Spring>, Vec<usize>), usize>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn part_two(input: &str) -> Option<u64> {
    let mut records = parse_input(input);

    extend_records(&mut records, 5);

    let res = records
        .par_iter()
        .progress()
        .map(|rec| count(rec.condition.clone(), rec.damaged.clone()))
        .collect::<Vec<usize>>();

    debug!("Count: {:?}", res);

    Some(res.iter().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{warn, Level};
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn test_part_one() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            warn!("setting default subscriber failed: {:?}", e)
        }

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            warn!("setting default subscriber failed: {:?}", e)
        }

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
