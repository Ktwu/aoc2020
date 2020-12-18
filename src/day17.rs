#![allow(dead_code)]

use crate::utils;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct ConwayGrid<const N: usize> {
    active_cells: HashSet<[i32; N]>,
}

impl<const N: usize> ConwayGrid<{ N }> {
    pub fn load(filename: &str) -> ConwayGrid<{ N }> {
        let input = utils::get_input(filename);

        ConwayGrid {
            active_cells: input
                .enumerate()
                .flat_map(|(i, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(j, c)| match c {
                            '#' => {
                                let mut coordinates = [0; N];
                                coordinates[N - 1] = j as i32;
                                coordinates[N - 2] = i as i32;
                                Some(coordinates)
                            }
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>(),
        }
    }

    fn into_iter(mut self) -> impl Iterator<Item = usize> {
        std::iter::from_fn(move || Some(self.step()))
    }

    fn step(&mut self) -> usize {
        let mut cells = HashMap::new();
        for cell in self
            .active_cells
            .iter()
            .flat_map(|cell| Self::neighbors(cell))
        {
            let count = cells.entry(cell).or_insert(0);
            *count += 1;
        }

        self.active_cells = cells
            .into_iter()
            .filter_map(|(cell, count)| {
                if count == 3 || (count == 2 && self.active_cells.contains(&cell)) {
                    Some(cell)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        self.active_cells.len()
    }

    fn neighbors(coordinates: &[i32; N]) -> impl Iterator<Item = [i32; N]> + '_ {
        let base_coordinates = coordinates.clone();
        std::iter::repeat(-1..=1)
            .take(N)
            .multi_cartesian_product()
            .filter_map(move |delta| {
                let mut neighbor = base_coordinates.clone();
                for i in 0..N {
                    neighbor[i] += delta[i];
                }
                if neighbor == base_coordinates {
                    None
                } else {
                    Some(neighbor)
                }
            })
    }
}

pub fn day17() {
    let mut conwaygrid = ConwayGrid::<3>::load("day17").into_iter();
    println!("conway_cubes part 1: {:?}", conwaygrid.nth(5).unwrap());

    let mut conwaygrid = ConwayGrid::<4>::load("day17").into_iter();
    println!("conway_cubes part 2: {:?}", conwaygrid.nth(5).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::day17::ConwayGrid;

    #[test]
    pub fn test_day17() {
        let mut conwaygrid = ConwayGrid::<3>::load("test_day17").into_iter();
        assert_eq!(conwaygrid.nth(1).unwrap(), 21);

        let mut conwaygrid = ConwayGrid::<4>::load("test_day17").into_iter();
        assert_eq!(conwaygrid.nth(5).unwrap(), 848);
    }
}
