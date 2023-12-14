use itertools::iproduct;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};
use tracing::{debug, info};

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Point,
    Galaxy(i64),
}

impl Tile {
    fn is_galaxy(&self) -> bool {
        match self {
            Self::Galaxy(_) => true,
            _ => false,
        }
    }

    fn get_galaxy(&self) -> i64 {
        match self {
            Self::Galaxy(id) => *id,
            _ => panic!("Not a galaxy!"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Clone)]
struct Expansion {
    pub row: Vec<usize>,
    pub col: Vec<usize>,
}

impl Expansion {
    pub fn new() -> Self {
        Self {
            row: Vec::new(),
            col: Vec::new(),
        }
    }

    pub fn add_row(&mut self, x: usize) {
        self.row.push(x);
    }

    pub fn add_col(&mut self, y: usize) {
        self.col.push(y);
    }
}

#[derive(Debug, Clone)]
struct Map {
    pub grid: Grid,
    pub has_expanded: bool,
    pub galaxy_numbers: i64,
    pub expansion: Expansion,
    pub extender: usize,
    pub is_part1: bool,
}

impl Map {
    pub fn new(input: &str, extender: usize, is_part1: bool) -> Self {
        let (grid, galaxy_numbers) = parse_input(input);

        Map {
            grid,
            has_expanded: false,
            galaxy_numbers,
            expansion: Expansion::new(),
            extender,
            is_part1,
        }
    }

    pub fn expand_with_expansion(&mut self) {
        if self.has_expanded {
            info!("Grid already expanded");
            return;
        }

        let row_expansions: Vec<usize> = self
            .grid
            .par_iter_mut()
            .enumerate()
            .filter_map(|(ind, row)| {
                if row.par_iter().all(|tile| !tile.is_galaxy()) {
                    return Some(ind);
                } else {
                    return None;
                }
            })
            .collect();

        let col_expansions: Vec<usize> = (0..self.grid[0].len())
            .filter(|&col_ind| self.grid.par_iter().all(|row| !row[col_ind].is_galaxy()))
            .collect();

        row_expansions.iter().for_each(|ind| {
            self.expansion.add_row(*ind);
        });

        col_expansions.iter().for_each(|col_ind| {
            self.expansion.add_col(*col_ind);
        });

        self.has_expanded = true;
    }

    fn get_galaxy_location(&self, galaxy: i64) -> (usize, usize) {
        self.grid
            .par_iter()
            .enumerate()
            .find_map_first(|(y, row)| {
                row.par_iter()
                    .enumerate()
                    .filter(|(_, tile)| tile.is_galaxy())
                    .find_map_first(|(x, tile)| {
                        if tile.get_galaxy() == galaxy {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
            })
            .unwrap()
    }

    fn get_expansions_multiplier(
        &self,
        fst: (usize, usize),
        snd: (usize, usize),
    ) -> (usize, usize) {
        let (min_x, max_x) = if fst.0 < snd.0 {
            (fst.0, snd.0)
        } else {
            (snd.0, fst.0)
        };

        let (min_y, max_y) = if fst.1 < snd.1 {
            (fst.1, snd.1)
        } else {
            (snd.1, fst.1)
        };

        let multiplier_x = self
            .expansion
            .clone()
            .row
            .into_par_iter()
            .filter(|&value| value >= min_y && value <= max_y)
            .count();

        let multiplier_y = self
            .expansion
            .clone()
            .col
            .into_par_iter()
            .filter(|&value| value >= min_x && value <= max_x)
            .count();

        if self.is_part1 {
            return (multiplier_x, multiplier_y);
        }

        (
            (multiplier_x * self.extender).abs_diff(multiplier_x),
            (multiplier_y * self.extender).abs_diff(multiplier_y),
        )
    }

    fn get_distance(&self, gal_1: i64, gal_2: i64) -> i64 {
        let (x1, y1) = self.get_galaxy_location(gal_1);
        let (x2, y2) = self.get_galaxy_location(gal_2);

        let (multiplier_x, multiplier_y) = self.get_expansions_multiplier((x1, y1), (x2, y2));

        debug!(
            "Distance between {} ({x1},{y1}) and {} ({x2},{y2}) with multiplier ({multiplier_x},{multiplier_y}) is {}",
            gal_1,
            gal_2,
            (((x1).abs_diff(x2) + multiplier_x) + (y1.abs_diff(y2) + multiplier_y))
        );

        (((x1).abs_diff(x2) + multiplier_x) + (y1.abs_diff(y2) + multiplier_y)) as i64
    }

    pub fn shortest_paths(&self) -> Vec<i64> {
        let all_galaxy_pairs: Vec<(i64, i64)> =
            iproduct!(1..self.galaxy_numbers, 2..=self.galaxy_numbers)
                .filter(|&(a, b)| a < b)
                .collect();

        let res: Vec<i64> = all_galaxy_pairs
            .par_iter()
            .map(|(galaxy_1, galaxy_2)| self.get_distance(*galaxy_1 as i64, *galaxy_2 as i64))
            .collect();

        debug!("all paths {:?}", res);

        res
    }

    // pub fn print(&self) {
    //     self.grid.par_iter().enumerate().for_each(|(_, row)| {
    //         row.par_iter().enumerate().for_each(|(_, tile)| match tile {
    //             Tile::Galaxy(i) => print!("{i}"),
    //             Tile::Point => print!("."),
    //         });

    //         println!("");
    //     });
    // }
}

fn parse_input(input: &str) -> (Grid, i64) {
    let mut counter = 0;

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Point,
                    '#' => {
                        counter += 1;
                        return Tile::Galaxy(counter);
                    }
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect();

    (grid, counter)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Map::new(input, 1, true);

    map.expand_with_expansion();

    let res = map.shortest_paths();

    Some(res.par_iter().sum::<i64>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::new(input, 1000000, false); //100 for test part 2

    map.expand_with_expansion();

    let res = map.shortest_paths();

    Some(res.par_iter().sum::<i64>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn test_part_one() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
