#![allow(dead_code)]

use crate::utils;

#[derive(Default, Clone, Copy, Debug)]
pub struct Cell {
    active: bool,
    future_active: bool,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Coordinates {
    w: usize,
    x: usize,
    y: usize,
    z: usize,
}

type CellPlane = Vec<Vec<Cell>>;
type CellGrid = Vec<CellPlane>;

pub struct ConwayGrid {
    cells: Vec<CellGrid>,
    hypercube: bool,
    need_grow: bool,
    num_w: usize,
    num_x: usize,
    num_y: usize,
    num_z: usize,
}

impl ConwayGrid {
    pub fn load(filename: &str, hypercube: bool) -> ConwayGrid {
        let input = utils::get_input(filename);

        let grid = vec![input
            .map(|line| {
                line.chars()
                    .map(|c| Cell {
                        active: c == '#',
                        future_active: false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()];
        let cells = vec![grid];

        ConwayGrid {
            num_w: cells.len(),
            num_x: cells[0].len(),
            num_y: cells[0][0].len(),
            num_z: cells[0][0][0].len(),
            need_grow: true,
            hypercube,
            cells,
        }
    }

    pub fn to_iter(mut self) -> impl Iterator<Item = usize> {
        std::iter::from_fn(move || Some(self.step()))
    }

    pub fn step(&mut self) -> usize {
        let mut active_count = 0;

        if self.need_grow {
            self.grow();
        }

        for w in 0..self.num_w {
            for x in 0..self.num_x {
                for y in 0..self.num_y {
                    for z in 0..self.num_z {
                        let active_count = self.active_neighbors(Coordinates { w, x, y, z });
                        let cell = &mut self.cells[w][x][y][z];
                        cell.future_active =
                            active_count == 3 || (cell.active && active_count == 2);
                    }
                }
            }
        }

        for w in 0..self.num_w {
            for x in 0..self.num_x {
                for y in 0..self.num_y {
                    for z in 0..self.num_z {
                        let cell = &mut self.cells[w][x][y][z];
                        cell.active = cell.future_active;

                        if cell.active {
                            active_count += 1;
                            self.need_grow |= w == 0
                                || x == 0
                                || y == 0
                                || z == 0
                                || w == self.num_w - 1
                                || x == self.num_x - 1
                                || y == self.num_y - 1
                                || z == self.num_z - 1;
                        }
                    }
                }
            }
        }

        active_count
    }

    pub fn active_neighbors(&self, coordinates: Coordinates) -> usize {
        let mut active_count = 0;
        for w in -1..=1 {
            let new_w = w + coordinates.w as i32;
            if !(0..self.num_w as i32).contains(&new_w) {
                continue;
            }

            for x in -1..=1 {
                let new_x = x + coordinates.x as i32;
                if !(0..self.num_x as i32).contains(&new_x) {
                    continue;
                }

                for y in -1..=1 {
                    let new_y = y + coordinates.y as i32;
                    if !(0..self.num_y as i32).contains(&new_y) {
                        continue;
                    }

                    for z in -1..=1 {
                        let new_z = z + coordinates.z as i32;
                        if !(0..self.num_z as i32).contains(&new_z) {
                            continue;
                        }

                        if !(x == 0 && y == 0 && z == 0 && w == 0)
                            && self.cells
                                [new_w as usize]
                                [new_x as usize]
                                [new_y as usize]
                                [new_z as usize]
                                .active
                        {
                            active_count += 1;
                        }
                    }
                }
            }
        }
        active_count
    }

    pub fn grow(&mut self) {
        self.num_z += 2;
        for w in 0..self.num_w {
            for x in 0..self.num_x {
                for y in 0..self.num_y {
                    let col = &mut self.cells[w][x][y];
                    col.push(Cell::default());
                    col.insert(0, Cell::default());
                }
            }
        }

        self.num_y += 2;
        for w in 0..self.num_w {
            for x in 0..self.num_x {
                let new_row = self.new_row();
                let plane = &mut self.cells[w][x];
                plane.push(new_row.clone());
                plane.insert(0, new_row);
            }
        }

        self.num_x += 2;
        for w in 0..self.num_w {
            let new_plane = self.new_plane();
            let grid = &mut self.cells[w];
            grid.push(new_plane.clone());
            grid.insert(0, new_plane);
        }

        if self.hypercube {
            let new_grid = self.new_grid();
            self.cells.push(new_grid.clone());
            self.cells.insert(0, new_grid);
            self.num_w += 2;
        }
    }

    fn new_grid(&self) -> CellGrid {
        let mut planes = Vec::with_capacity(self.num_x);
        for _ in 0..self.num_x {
            planes.push(self.new_plane())
        }
        planes
    }

    fn new_plane(&self) -> CellPlane {
        let mut rows = Vec::with_capacity(self.num_y);
        for _ in 0..self.num_y {
            rows.push(self.new_row());
        }
        rows
    }

    fn new_row(&self) -> Vec<Cell> {
        vec![Cell::default(); self.num_z]
    }
}

pub fn day17() {
    let mut conwaygrid = ConwayGrid::load("day17", false).to_iter();
    println!("conway_cubes part 1: {:?}", conwaygrid.nth(5).unwrap());

    let mut conwaygrid = ConwayGrid::load("day17", true).to_iter();
    println!("conway_cubes part 2: {:?}", conwaygrid.nth(5).unwrap());
}

#[test]
pub fn test_day17() {
    let mut conwaygrid = ConwayGrid::load("test_day17", false).to_iter();
    assert_eq!(conwaygrid.nth(1).unwrap(), 21);

    let mut conwaygrid = ConwayGrid::load("test_day17", true).to_iter();
    assert_eq!(conwaygrid.nth(5).unwrap(), 848);
}
