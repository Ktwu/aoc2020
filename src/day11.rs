#![allow(dead_code)]

use crate::{utils};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SeatState {
    Floor,
    Empty,
    Occupied,
}

type SeatStepMap = Vec<Vec<Vec<Option<(usize, usize)>>>>;

pub struct SeatGrid {
    grid: Vec<Vec<SeatState>>,
    num_rows: usize,
    num_columns: usize,
}

impl SeatGrid {
    pub fn load(filename: &str) -> SeatGrid {
        let input = utils::get_input(filename);
        let mut grid: Vec<Vec<SeatState>> = Vec::new();

        for line in input {
            grid.push(line.chars().map(|seat| 
                match seat {
                    'L' => SeatState::Empty,
                    '#' => SeatState::Occupied,
                    _ => SeatState::Floor,
                }).collect()
            );
        }

        SeatGrid {
            num_columns: grid[0].len(),
            num_rows: grid.len(),
            grid
        }
   }

   pub fn stabilize_immediate(&mut self) -> usize {
       let mut step_map: SeatStepMap = Vec::new();

       for (y, row) in self.grid.iter().enumerate() {
           let mut new_row = Vec::new();
           for (x, _seat) in row.iter().enumerate() {
            new_row.push([
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ].iter().map(|(dx, dy)| self.in_grid(
                dx + x as i32,
                dy + y as i32,
            )).collect());
           }
           step_map.push(new_row);
       }

       self.stabilize(step_map)
   }

    fn stabilize(&mut self, step_map: SeatStepMap) -> usize {
        let mut cache = self.grid.clone();
        loop {
            let mut did_update = false;
            let mut occupied_count = 0;

            for (y, row) in self.grid.iter().enumerate() {
                for (x, seat) in row.iter().enumerate() {
                    let (num_occupied, _num_empty) = step_map[x][y].iter().map(|cell|
                        match cell {
                            Some((i, j)) => self.grid[*i][*j],
                            None => SeatState::Floor,
                        }
                    ).fold((0, 0), |(ocount, ecount), seat| match seat {
                        SeatState::Empty => (ocount, ecount+1),
                        SeatState::Occupied => (ocount+1, ecount),
                        SeatState::Floor => (ocount, ecount),
                    });

                    cache[y][x] = match seat {
                        SeatState::Empty => if num_occupied == 0 { SeatState::Occupied } else { SeatState::Empty },
                        SeatState::Occupied => if num_occupied >= 4 {SeatState::Empty } else { SeatState::Occupied },
                        SeatState::Floor => SeatState::Floor,
                    };

                    if cache[y][x] == SeatState::Occupied {
                        occupied_count += 1;
                    }

                    did_update = did_update || self.grid[y][x] != cache[y][x];
                }
            }

            if did_update {
                self.grid = cache.clone();
            } else {
                return occupied_count;
            }
        }
    }

    pub fn in_grid(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        if x < 0 || y < 0 || x >= self.num_columns as i32 || y >= self.num_rows as i32 {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    pub fn find_next_seat(&self, cell: (i32, i32), delta: (i32, i32)) -> Option<(usize, usize)> {
        let mut candidate = (cell.0 + delta.0, cell.1 + delta.1);
        loop {
            let cell = self.in_grid(candidate.0, candidate.1);
            if let Some((x, y)) = cell {
                if self.grid[x][y] != SeatState::Floor {
                    break cell;
                }
                candidate.0 += delta.0;
                candidate.1 += delta.1;
            } else {
                break None;
            }
        }
    }
}

pub fn day11() {
    let mut grid = SeatGrid::load("day11");
    println!("seating_system part 1: {:?}", grid.stabilize_immediate());
    //println!("adaptor_array part 2: {:?}", count_arrangements(&adaptors));
}

#[test]
pub fn test_day11() {
    let mut input = SeatGrid::load("test_day11");
    assert_eq!(
        input.stabilize_immediate(),
        37,
    );
}
