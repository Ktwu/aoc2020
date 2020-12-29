#![allow(dead_code)]

use crate::utils;
use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum Direction {
    E,
    NE,
    SE,
    W,
    NW,
    SW,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::E => (2, 0),
            Direction::W => (-2, 0),
            Direction::NE => (1, 1),
            Direction::NW => (-1, 1),
            Direction::SE => (1, -1),
            Direction::SW => (-1, -1),
        }
    }

    pub fn parse(input: &mut impl Iterator<Item = char>) -> Option<(i32, i32)> {
        Some(
            match input.next()? {
                'e' => Direction::E,
                'w' => Direction::W,
                'n' => match input.next()? {
                    'e' => Direction::NE,
                    _ => Direction::NW,
                },
                _ => match input.next()? {
                    'e' => Direction::SE,
                    _ => Direction::SW,
                },
            }
            .delta(),
        )
    }
}

pub struct TileFloor {
    black_tiles: HashSet<(i32, i32)>,
}

impl TileFloor {
    pub fn load(filename: &str) -> TileFloor {
        let mut black_tiles = HashSet::new();
        let input = utils::get_input(filename);

        for line in input {
            let tile = TileFloor::parse_tile(&line);
            if !black_tiles.remove(&tile) {
                black_tiles.insert(tile);
            }
        }

        TileFloor { black_tiles }
    }

    pub fn parse_tile(line: &str) -> (i32, i32) {
        let mut coordinates = (0, 0);
        let mut chars = line.chars();
        while let Some(direction) = Direction::parse(&mut chars) {
            coordinates.0 += direction.0;
            coordinates.1 += direction.1;
        }
        coordinates
    }

    pub fn num_black(&self) -> usize {
        self.black_tiles.len()
    }

    pub fn step(&mut self) {
        let mut neighbor_count: HashMap<(i32, i32), usize> = HashMap::new();
        for tile in self.black_tiles.iter() {
            neighbor_count.entry(*tile).or_insert(0);

            for neighbor in Direction::iter().map(|v| {
                let delta = v.delta();
                (tile.0 + delta.0, tile.1 + delta.1)
            }) {
                *neighbor_count.entry(neighbor).or_insert(0) += 1;
            }
        }

        self.black_tiles = neighbor_count
            .into_iter()
            .filter_map(|(tile, count)|
                if count == 2 || (self.black_tiles.contains(&tile) && count == 1) {
                    Some(tile)
                } else {
                    None
                }
            )
            .collect::<HashSet<_>>();
    }
}

pub fn day24() {
    let mut tiles = TileFloor::load("day24");
    println!("lobby_layout part 1: {:?}", tiles.num_black());

    for _ in 0..100 {
        tiles.step();
    }
    println!("lobby_layout part 2: {:?}", tiles.num_black());

}

#[cfg(test)]
mod tests {
    use crate::day24::*;

    #[test]
    pub fn test_day24() {
        assert_eq!((0, 0), TileFloor::parse_tile("nwwswee"));

        let mut tiles = TileFloor::load("test_day24");
        assert_eq!(tiles.num_black(), 10);

        tiles.step();
        assert_eq!(tiles.num_black(), 15);

        tiles.step();
        assert_eq!(tiles.num_black(), 12);
    
        for _ in 0..98 {
            tiles.step();
        }
        assert_eq!(tiles.num_black(), 2208);
    }
}
