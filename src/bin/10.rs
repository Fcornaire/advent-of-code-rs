advent_of_code::solution!(10);
use std::{collections::HashSet, path};
use tracing::Level;
use tracing::{debug, info, instrument};
use tracing_subscriber::FmtSubscriber;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Vertical(CardinalDirection, CardinalDirection),
    Horizontal(CardinalDirection, CardinalDirection),
    L(CardinalDirection, CardinalDirection),
    J(CardinalDirection, CardinalDirection),
    Seven(CardinalDirection, CardinalDirection),
    F(CardinalDirection, CardinalDirection),
}

impl Pipe {
    pub fn new(first: CardinalDirection, second: CardinalDirection) -> Self {
        match (first, second) {
            (CardinalDirection::North, CardinalDirection::South)
            | (CardinalDirection::South, CardinalDirection::North) => {
                Self::Vertical(CardinalDirection::North, CardinalDirection::South)
            }
            (CardinalDirection::East, CardinalDirection::West)
            | (CardinalDirection::West, CardinalDirection::East) => {
                Self::Horizontal(CardinalDirection::East, CardinalDirection::West)
            }
            (CardinalDirection::North, CardinalDirection::East)
            | (CardinalDirection::East, CardinalDirection::North) => {
                Self::L(CardinalDirection::North, CardinalDirection::East)
            }
            (CardinalDirection::North, CardinalDirection::West)
            | (CardinalDirection::West, CardinalDirection::North) => {
                Self::J(CardinalDirection::North, CardinalDirection::West)
            }
            (CardinalDirection::South, CardinalDirection::West)
            | (CardinalDirection::West, CardinalDirection::South) => {
                Self::Seven(CardinalDirection::South, CardinalDirection::West)
            }
            (CardinalDirection::South, CardinalDirection::East)
            | (CardinalDirection::East, CardinalDirection::South) => {
                Self::F(CardinalDirection::South, CardinalDirection::East)
            }
            _ => panic!("Invalid pipe: {:?}, {:?}", first, second),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Vertical(CardinalDirection::North, CardinalDirection::South),
            Self::Horizontal(CardinalDirection::East, CardinalDirection::West),
            Self::L(CardinalDirection::North, CardinalDirection::East),
            Self::J(CardinalDirection::North, CardinalDirection::West),
            Self::Seven(CardinalDirection::South, CardinalDirection::West),
            Self::F(CardinalDirection::South, CardinalDirection::East),
        ]
    }

    pub fn get_directions(&self) -> (&CardinalDirection, &CardinalDirection) {
        match self {
            Self::Vertical(first, second) => (first, second),
            Self::Horizontal(first, second) => (first, second),
            Self::L(first, second) => (first, second),
            Self::J(first, second) => (first, second),
            Self::Seven(first, second) => (first, second),
            Self::F(first, second) => (first, second),
        }
    }

    fn can_connect_x(&self) -> bool {
        match self {
            Self::Vertical(_, _) => false,
            Self::Horizontal(_, _) => true,
            Self::L(_, _) => true,
            Self::J(_, _) => true,
            Self::Seven(_, _) => true,
            Self::F(_, _) => true,
        }
    }

    fn can_connect_y(&self) -> bool {
        match self {
            Self::Vertical(_, _) => true,
            Self::Horizontal(_, _) => false,
            Self::L(_, _) => true,
            Self::J(_, _) => true,
            Self::Seven(_, _) => true,
            Self::F(_, _) => true,
        }
    }

    #[instrument]
    pub fn can_connect(&self, other: Self) -> bool {
        self.get_directions() == other.get_directions()
            || (self.can_connect_x() && other.can_connect_x())
            || (self.can_connect_y() && other.can_connect_y())
    }
}

fn get_connecting_pipe(fst: Pipe, snd: Pipe) -> Option<Pipe> {
    Pipe::all()
        .par_iter()
        .find_first(|pipe| {
            pipe.can_connect(fst) && pipe.can_connect(snd) && *pipe != &fst && *pipe != &snd
        })
        .copied()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Pipe(Pipe),
    S(Option<Pipe>),
}

impl Tile {
    pub fn get_pipe(&self) -> Option<Pipe> {
        match self {
            Self::Pipe(pipe) => Some(*pipe),
            Self::S(pipe) => *pipe,
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

    fn find_starting_position(&self) -> Option<(usize, usize)> {
        self.map.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if let Tile::S(_) = tile {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }

    pub fn update_starting_position(&mut self) {
        let (x, y) = self.find_starting_position().unwrap();
        let neighbors = self.get_neighbors(x as i32, y as i32);

        let pipe = neighbors.iter().combinations(2).find_map(|pair| {
            let pair_1 = pair[0].0.get_pipe();
            let pair_2 = pair[1].0.get_pipe();

            pair_1.and_then(|pipe1| pair_2.and_then(|pipe2| get_connecting_pipe(pipe1, pipe2)))
        });

        if let Some(pipe) = pipe {
            self.get_mut(x, y).map(|tile| *tile = Tile::S(Some(pipe)));
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y > self.map.len() as i32 {
            return None;
        }

        self.map.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.map.get_mut(y).and_then(|row| row.get_mut(x))
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

    fn get_connecting_neighbors(&self, x: i32, y: i32) -> Vec<(Tile, (i32, i32))> {
        let mut neighbors = Vec::new();
        let current_pipe = self.get(x, y).unwrap().get_pipe().unwrap();

        if let Some(tile) = self.get(x, y - 1) {
            if tile
                .get_pipe()
                .map_or(false, |pipe| current_pipe.can_connect(pipe))
            {
                neighbors.push((*tile, (x, y - 1)));
            }
        }
        if let Some(tile) = self.get(x, y + 1) {
            if tile
                .get_pipe()
                .map_or(false, |pipe| current_pipe.can_connect(pipe))
            {
                neighbors.push((*tile, (x, y + 1)));
            }
        }
        if let Some(tile) = self.get(x - 1, y) {
            if tile
                .get_pipe()
                .map_or(false, |pipe| current_pipe.can_connect(pipe))
            {
                neighbors.push((*tile, (x - 1, y)));
            }
        }
        if let Some(tile) = self.get(x + 1, y) {
            if tile
                .get_pipe()
                .map_or(false, |pipe| current_pipe.can_connect(pipe))
            {
                neighbors.push((*tile, (x + 1, y)));
            }
        }
        neighbors
    }

    fn get_path(&self, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>) {
        let neighbors = self.get_connecting_neighbors(x, y);

        visited.insert((x, y));

        if neighbors.is_empty() {
            return;
        }

        neighbors.iter().for_each(|(tile, (x, y))| {
            if let Some(pipe) = tile.get_pipe() {
                if visited.contains(&(*x, *y)) {
                    return;
                }

                self.get_path(*x, *y, visited);
            }
        })
    }

    fn get_loop_from_starting_position(&self) -> HashSet<(i32, i32)> {
        let (starting_x, starting_y) = self.find_starting_position().unwrap();
        let starting_pipe = self
            .get(starting_x as i32, starting_y as i32)
            .unwrap()
            .get_pipe()
            .unwrap();
        let neighbors = self.get_connecting_neighbors(starting_x as i32, starting_y as i32);

        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        visited.insert((starting_x as i32, starting_y as i32));

        neighbors.iter().for_each(|(tile, (x, y))| {
            if let Some(pipe) = tile.get_pipe() {
                self.get_path(*x, *y, &mut visited);
            }
        });

        visited
    }
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Ground,
                    '|' => Tile::Pipe(Pipe::new(
                        CardinalDirection::North,
                        CardinalDirection::South,
                    )),
                    '-' => Tile::Pipe(Pipe::new(CardinalDirection::East, CardinalDirection::West)),
                    'L' => Tile::Pipe(Pipe::new(CardinalDirection::North, CardinalDirection::East)),
                    'J' => Tile::Pipe(Pipe::new(CardinalDirection::North, CardinalDirection::West)),
                    '7' => Tile::Pipe(Pipe::new(CardinalDirection::South, CardinalDirection::West)),
                    'F' => Tile::Pipe(Pipe::F(CardinalDirection::South, CardinalDirection::East)),
                    'S' => Tile::S(None),
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut grid = Grid::new(parse_input(input));

    grid.update_starting_position();

    let paths = grid.get_loop_from_starting_position();
    info!("Main loop {:?}", paths);

    Some((paths.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
