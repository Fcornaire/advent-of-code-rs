advent_of_code::solution!(10);
use itertools::{iproduct, Itertools};
use std::{collections::HashSet, path};
use tracing::debug;
use tracing_subscriber::field::debug;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Vertical(char),
    Horizontal(char),
    L(char),
    J(char),
    Seven(char),
    F(char),
    S(char),
}

impl Pipe {
    pub fn get_char(&self) -> char {
        match self {
            Self::Vertical(ch) => *ch,
            Self::Horizontal(ch) => *ch,
            Self::L(ch) => *ch,
            Self::J(ch) => *ch,
            Self::Seven(ch) => *ch,
            Self::F(ch) => *ch,
            Self::S(ch) => *ch,
        }
    }
}

fn can_two_pipe_connect(fst: (Pipe, (i32, i32)), snd: (Pipe, (i32, i32))) -> bool {
    let (fst_pipe, (fst_x, fst_y)) = fst;
    let (snd_pipe, (snd_x, snd_y)) = snd;

    match ((fst_x, fst_y), (snd_x, snd_y)) {
        ((x, y), (x2, y2)) if x == x2 && y == y2 + 1 => {
            "S|JL".contains(fst_pipe.get_char()) && "S|7F".contains(snd_pipe.get_char())
        }
        ((x, y), (x2, y2)) if x == x2 && y == y2 - 1 => {
            "S|7F".contains(fst_pipe.get_char()) && "S|JL".contains(snd_pipe.get_char())
        }
        ((x, y), (x2, y2)) if x == x2 + 1 && y == y2 => {
            "S-J7".contains(fst_pipe.get_char()) && "S-LF".contains(snd_pipe.get_char())
        }
        ((x, y), (x2, y2)) if x == x2 - 1 && y == y2 => {
            "S-LF".contains(fst_pipe.get_char()) && "S-J7".contains(snd_pipe.get_char())
        }
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Pipe(Pipe),
}

impl Tile {
    pub fn get_pipe(&self) -> Option<Pipe> {
        match self {
            Self::Pipe(pipe) => Some(*pipe),
            _ => None,
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct Grid {
    map: Map,
}

impl Grid {
    fn new(map: Map) -> Self {
        Self { map }
    }

    fn find_starting_position(&self) -> Option<(i32, i32)> {
        self.map.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if let Tile::Pipe(Pipe::S(_)) = tile {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
    }

    fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y > self.map.len() as i32 {
            return None;
        }

        self.map.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn get_neighbors(&self, x: i32, y: i32) -> Vec<(Tile, (i32, i32))> {
        let mut neighbors = Vec::new();
        if let Some(tile) = self.get(x, y - 1) {
            neighbors.push((*tile, (x, y - 1)));
        }
        if let Some(tile) = self.get(x, y + 1) {
            neighbors.push((*tile, (x, y + 1)));
        }
        if let Some(tile) = self.get(x - 1, y) {
            neighbors.push((*tile, (x - 1, y)));
        }
        if let Some(tile) = self.get(x + 1, y) {
            neighbors.push((*tile, (x + 1, y)));
        }
        neighbors
    }

    fn get_loop_from_starting_position(&self) -> HashSet<(i32, i32)> {
        let (starting_x, starting_y) = self.find_starting_position().unwrap();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut stack = VecDeque::new();

        stack.push_back((starting_x, starting_y));

        while let Some((x, y)) = stack.pop_front() {
            visited.insert((x, y));

            let neighbors = self.get_neighbors(x, y);
            let nexts = neighbors
                .par_iter()
                .filter_map(|(tile, (nx, ny))| {
                    if tile.get_pipe().map_or(false, |pipe| {
                        !visited.contains(&(*nx, *ny))
                            && can_two_pipe_connect(
                                (pipe, (*nx, *ny)),
                                (self.get(x, y).unwrap().get_pipe().unwrap(), (x, y)),
                            )
                    }) {
                        Some((*nx, *ny))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i32, i32)>>();

            stack.extend(nexts);
            stack.retain(|(x, y)| !visited.contains(&(*x, *y)));
        }

        visited
    }

    fn get_enclosed_tiles(&self) -> HashSet<(i32, i32)> {
        let paths = self.get_loop_from_starting_position();

        let mut res: HashSet<(i32, i32)> = HashSet::new();
        let mut potential_enclosed_x: HashSet<(i32, i32)> = HashSet::new();
        let mut potential_enclosed_y: HashSet<(i32, i32)> = HashSet::new();

        debug!("Paths: {:?}", paths);

        self.map.iter().enumerate().for_each(|(y, row)| {
            let mut x = 0;
            let mut is_in_loop = false;

            while x < row.len() {
                let current_tile = row[x];

                if paths.contains(&(x as i32, y as i32)) {
                    let current_pipe = current_tile.get_pipe();
                    if let Some(pipe) = current_pipe {
                        if "SLF".contains(pipe.get_char()) {
                            is_in_loop = true;
                        }

                        if "SJ7".contains(pipe.get_char()) {
                            is_in_loop = false;
                        }

                        if "|".contains(pipe.get_char()) {
                            is_in_loop = !is_in_loop;
                        }
                    }
                } else {
                    if is_in_loop {
                        potential_enclosed_x.insert((x as i32, y as i32));
                    }
                }

                x += 1;
            }
        });

        for x in 0..self.map[0].len() {
            let mut y = 0;
            let mut is_in_loop = false;

            while y < self.map.len() {
                let current_tile = self.map[y][x];

                if paths.contains(&(x as i32, y as i32)) {
                    let current_pipe = current_tile.get_pipe();
                    if let Some(pipe) = current_pipe {
                        if "SLFJ7-".contains(pipe.get_char()) {
                            is_in_loop = !is_in_loop;
                        }
                    }
                } else {
                    if is_in_loop {
                        potential_enclosed_y.insert((x as i32, y as i32));
                    }
                }

                y += 1;
            }
        }

        debug!("Potential enclosed x: {:?}", potential_enclosed_x);
        debug!("Potential enclosed y: {:?}", potential_enclosed_y);

        // get (x, y) that are in both potential_enclosed_x and potential_enclosed_y
        potential_enclosed_x
            .intersection(&potential_enclosed_y)
            .for_each(|(x, y)| {
                res.insert((*x, *y));
            });

        debug!("Enclosed tiles: {:?}", res);

        if cfg!(debug_assertions) {
            self.print_grid(paths.clone(), res.clone());
        }

        res
    }

    fn print_grid(&self, paths: HashSet<(i32, i32)>, res: HashSet<(i32, i32)>) {
        self.map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, tile)| {
                print!(
                    "{}",
                    match tile {
                        Tile::Pipe(pipe) => {
                            if paths.contains(&(x as i32, y as i32)) {
                                'X'
                            } else if res.contains(&(x as i32, y as i32)) {
                                'I'
                            } else {
                                ' '
                            }
                        }
                        _ =>
                            if res.contains(&(x as i32, y as i32)) {
                                'I'
                            } else {
                                '.'
                            },
                    }
                )
            });
            println!();
        });
    }
}

fn contains_any<T: Eq + std::hash::Hash>(vec: &[T], set: &HashSet<T>) -> bool {
    vec.iter().any(|item| set.contains(item))
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Ground,
                    '|' => Tile::Pipe(Pipe::Vertical('|')),
                    '-' => Tile::Pipe(Pipe::Horizontal('-')),
                    'L' => Tile::Pipe(Pipe::L('L')),
                    'J' => Tile::Pipe(Pipe::J('J')),
                    '7' => Tile::Pipe(Pipe::Seven('7')),
                    'F' => Tile::Pipe(Pipe::F('F')),
                    'S' => Tile::Pipe(Pipe::S('S')),
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(parse_input(input));

    let paths = grid.get_loop_from_starting_position();
    debug!("Main loop {:#?}", paths);

    Some((paths.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(parse_input(input));

    let enclosed = grid.get_enclosed_tiles();
    // debug!("Main loop {:?}", paths);

    Some(enclosed.len() as u32)
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

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }
}
