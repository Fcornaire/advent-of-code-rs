use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use tracing::debug;

advent_of_code::solution!(13);

static SAVED_REFLECTIONS_ROW_CHECK: Lazy<Arc<Mutex<HashMap<(usize, (usize, usize)), usize>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

static SAVED_REFLECTIONS_COLUMN_CHECK: Lazy<Arc<Mutex<HashMap<(usize, (usize, usize)), usize>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Ash,
    Rock,
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Element>>> {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Element::Rock,
                            '.' => Element::Ash,
                            _ => panic!("Invalid character in input"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn get_element_at_column(elementss: Vec<Vec<Element>>, column: usize) -> Vec<Element> {
    elementss
        .iter()
        .map(|elements| elements[column])
        .collect::<Vec<Element>>()
}

fn is_reflection(fst_elements: Vec<Element>, snd_elements: Vec<Element>) -> bool {
    fst_elements
        .iter()
        .zip(snd_elements.iter())
        .all(|(fst, snd)| fst == snd)
}

fn count_row_reflection(
    elementss: Vec<Vec<Element>>,
    index_to_check: (i32, i32),
    reference: usize,
) -> usize {
    let (ind_1, ind_2) = index_to_check;

    if ind_1 < 0 || ind_2 as usize >= elementss.len() {
        return 0;
    }

    let key = (reference, (ind_1 as usize, ind_2 as usize));
    if let Some(cached) = SAVED_REFLECTIONS_ROW_CHECK.lock().unwrap().get(&key) {
        return *cached;
    }

    let mut count = 0;

    if is_reflection(
        elementss[ind_1 as usize].clone(),
        elementss[ind_2 as usize].clone(),
    ) {
        count += 1;

        let next_ind_1 = ind_1 - 1;
        let next_ind_2 = ind_2 + 1;

        if next_ind_1 >= 0 && (next_ind_2 as usize) < elementss.len() {
            count += count_row_reflection(elementss, (next_ind_1, next_ind_2), reference);
        }
    }

    SAVED_REFLECTIONS_ROW_CHECK
        .lock()
        .unwrap()
        .insert(key, count);

    count
}

fn count_column_reflection(
    elementss: Vec<Vec<Element>>,
    index_to_check: (i32, i32),
    reference: usize,
) -> usize {
    let (ind_1, ind_2) = index_to_check;

    if ind_1 < 0 || ind_2 as usize >= elementss[0].len() {
        return 0;
    }

    let key = (reference, (ind_1 as usize, ind_2 as usize));
    if let Some(cached) = SAVED_REFLECTIONS_COLUMN_CHECK.lock().unwrap().get(&key) {
        return *cached;
    }

    let mut count = 0;

    if is_reflection(
        get_element_at_column(elementss.clone(), ind_1 as usize),
        get_element_at_column(elementss.clone(), ind_2 as usize),
    ) {
        count += 1;

        let next_ind_1 = ind_1 - 1;
        let next_ind_2 = ind_2 + 1;

        if next_ind_1 >= 0 && (next_ind_2 as usize) < elementss[0].len() {
            count += count_column_reflection(elementss, (next_ind_1, next_ind_2), reference);
        }
    }

    SAVED_REFLECTIONS_COLUMN_CHECK
        .lock()
        .unwrap()
        .insert(key, count);

    count
}

fn get_row_reflection_incidence(elementss: Vec<Vec<Element>>, ind: usize) -> ((i32, i32), usize) {
    let counter: Arc<Mutex<Vec<((i32, i32), usize)>>> = Arc::new(Mutex::new(vec![]));

    (0..elementss.len())
        // .combinations(2)
        .tuple_windows()
        .par_bridge()
        .for_each(|combination: (usize, usize)| {
            let (fst, snd) = (combination.0, combination.1);

            let res = count_row_reflection(elementss.clone(), (fst as i32, snd as i32), ind);

            counter
                .clone()
                .lock()
                .unwrap()
                .push(((fst as i32, snd as i32), res));
        });

    {
        let clone = counter.clone();

        let tmp = clone.lock().unwrap();

        let superiors = tmp
            .iter()
            .filter(|(_, count)| *count > 0)
            .collect::<Vec<&((i32, i32), usize)>>();

        debug!("superiors row: {:?}", superiors);
    }

    if counter.clone().lock().unwrap().is_empty() {
        return ((-1, -1), 0);
    }

    counter
        .clone()
        .lock()
        .unwrap()
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(index, count)| (*index, *count))
        .unwrap()
}

fn get_column_reflection_incidence(
    elementss: Vec<Vec<Element>>,
    ind: usize,
) -> ((i32, i32), usize) {
    let counter: Arc<Mutex<Vec<((i32, i32), usize)>>> = Arc::new(Mutex::new(vec![]));

    (0..elementss.len())
        // .combinations(2)
        .tuple_windows()
        .par_bridge()
        .for_each(|combination: (usize, usize)| {
            let (fst, snd) = (combination.0, combination.1);

            let res = count_column_reflection(elementss.clone(), (fst as i32, snd as i32), ind);

            // debug!("index: {:?}, res: {}", (fst, snd), res);

            counter
                .clone()
                .lock()
                .unwrap()
                .push(((fst as i32, snd as i32), res));
        });

    {
        let clone = counter.clone();

        let tmp = clone.lock().unwrap();

        let superiors = tmp
            .iter()
            .filter(|(_, count)| *count > 0)
            .collect::<Vec<&((i32, i32), usize)>>();

        debug!("superiors column: {:?}", superiors);
    }

    if counter.clone().lock().unwrap().is_empty() {
        return ((-1, -1), 0);
    }

    counter
        .clone()
        .lock()
        .unwrap()
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(index, count)| (*index, *count))
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    // debug!("Grid: {:?}", grid);

    let incidence_rows = grid
        .par_iter()
        .progress()
        .enumerate()
        .map(|(ind, elements)| get_row_reflection_incidence(elements.clone(), ind))
        .collect::<Vec<((i32, i32), usize)>>();

    // debug!("Incidence rows: {:?}", incidence_rows);

    let incidence_columns = grid
        .par_iter()
        .progress()
        .enumerate()
        .map(|(ind, elements)| get_column_reflection_incidence(elements.clone(), ind))
        .collect::<Vec<((i32, i32), usize)>>();

    // debug!("Incidence columns: {:?}", incidence_columns);

    let max_incidence: Vec<((i32, i32), usize, i32)> = incidence_rows
        .iter()
        .zip(incidence_columns.iter())
        .filter(|((_, row_count), (_, column_count))| *row_count > 0 && *column_count > 0)
        .map(|((row_index, row_count), (column_index, column_count))| {
            if row_count > column_count {
                (*row_index, *row_count, ((*row_index).0 + 1) * 100)
            } else {
                (*column_index, *column_count, ((*column_index).0 + 1))
            }
        })
        .collect();

    debug!("Max incidence: {:?}", max_incidence);

    let res: i32 = max_incidence.iter().map(|(_, _, res)| res).sum();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, Some(405));
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
        assert_eq!(result, None);
    }
}
