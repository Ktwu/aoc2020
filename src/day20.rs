#![allow(dead_code)]

use crate::{regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use utils::AOCResult;

lazy_static! {
    static ref ID_REGEX: Regex = regex!(r"^Tile ([0-9]+):");
}

///
/// we have a bunch of tiles
/// each needs to be flipped or rotated so that edges line up in some way, shape, or form with other tiles
/// first: parse out each tile
/// for now we'll ignore the actual image content (we'll probably need it later tbh)
///
/// We have a tile map with some idea of associations; we can reconstruct our final image one row at a time.
///   Figure out which tiles are Corners
///   Per Corner:
///
///

type TileEdge = [bool; 10];

#[derive(Default, Debug)]
pub struct TileTransform {
    flipped_edge: Option<usize>,
    rotate_steps: usize,
    relative_position: (i8, i8),
}

pub struct TileEdgeMatch {
    edge_id: usize,
    flipped: bool,
}

#[derive(Clone)]
pub struct Tile {
    id: usize,
    image_content: [TileEdge; 10],
    tile_edges: [TileEdge; 4],
    flipped_edges: [TileEdge; 4],
}

pub struct TilePuzzle {
    tiles: HashMap<usize, Tile>,
    associations: HashMap<usize, Vec<usize>>,
}

pub struct TileSolution {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

pub struct TileImage {
    pixels: Vec<Vec<bool>>,
}

impl Tile {
    pub fn load(input: &mut impl Iterator<Item = String>) -> AOCResult<Tile> {
        let id = ID_REGEX.captures_iter(&input.next()?).next()?[1].parse::<usize>()?;
        let mut image_content = [[false; 10]; 10];
        let mut i = 0;
        for line in input {
            if line.len() == 0 {
                break;
            }
            for (j, c) in line.chars().enumerate() {
                image_content[i][j] = c == '#';
            }
            i += 1;
        }

        let mut tile = Tile {
            id,
            image_content,
            tile_edges: [[false; 10]; 4],
            flipped_edges: [[false; 10]; 4],
        };

        tile.update_edges();
        Ok(tile)
    }

    fn update_edges(&mut self) {
        for i in 0..10 {
            self.tile_edges[0][i] = self.image_content[0][i];
            self.tile_edges[1][i] = self.image_content[9 - i][0];
            self.tile_edges[2][i] = self.image_content[9][9 - i];
            self.tile_edges[3][i] = self.image_content[i][9];
        }
        for i in 0..10 {
            for j in 0..4 {
                self.flipped_edges[j][i] = self.tile_edges[j][9 - i];
            }
        }
    }

    pub fn can_transform(&self, other: &Tile) -> Option<TileTransform> {
        for i in 0..4 {
            if let Some(edge_match) = self.can_match(&other.tile_edges[i]) {
                return Some(self.transform_info(edge_match, i));
            }
        }
        None
    }

    pub fn can_match(&self, edge: &TileEdge) -> Option<TileEdgeMatch> {
        for edge_id in 0..4 {
            if self.flipped_edges[edge_id] == *edge {
                return Some(TileEdgeMatch {
                    edge_id,
                    flipped: false,
                });
            }
            if self.tile_edges[edge_id] == *edge {
                return Some(TileEdgeMatch {
                    edge_id,
                    flipped: true,
                });
            }
        }
        None
    }

    pub fn flip_over_edge(&mut self, i: usize) {
        let tmp_image = self.image_content;
        for y in 0..10 {
            for x in 0..10 {
                if i % 2 == 0 {
                    self.image_content[y][x] = tmp_image[y][9 - x];
                } else {
                    self.image_content[y][x] = tmp_image[9 - y][x];
                }
            }
        }
        self.update_edges();
    }

    pub fn rotate_clockwise(&mut self, steps: usize) {
        let tmp_image = self.image_content;
        for i in 0..10 {
            for j in 0..10 {
                self.image_content[i][j] = match steps % 4 {
                    1 => tmp_image[9 - j][i],
                    2 => tmp_image[9 - i][9 - j],
                    3 => tmp_image[j][9 - i],
                    _ => tmp_image[i][j],
                };
            }
        }
        self.update_edges();
    }

    fn transform_info(&self, edge_match: TileEdgeMatch, edge_to_match: usize) -> TileTransform {
        let align_edge_id = (edge_to_match + 2) % 4;
        let mut steps = 0;
        while (edge_match.edge_id + 4 - steps) % 4 != align_edge_id {
            steps += 1;
        }
        TileTransform {
            flipped_edge: if edge_match.flipped {
                Some(edge_match.edge_id)
            } else {
                None
            },
            rotate_steps: steps,
            relative_position: match edge_to_match {
                0 => (0, 1),
                1 => (-1, 0),
                2 => (0, -1),
                _ => (1, 0),
            },
        }
    }

    pub fn apply(&mut self, transform: &TileTransform) {
        if let Some(id) = transform.flipped_edge {
            self.flip_over_edge(id);
        }

        if transform.rotate_steps > 0 {
            self.rotate_clockwise(transform.rotate_steps);
        }
    }
}

impl TilePuzzle {
    pub fn load(filename: &str) -> AOCResult<TilePuzzle> {
        let tiles = Self::load_tiles(filename);
        let associations = Self::tile_associations(&tiles);
        Ok(TilePuzzle {
            tiles,
            associations,
        })
    }

    pub fn solve(mut self) -> AOCResult<TileSolution> {
        let mut corners = self.corners();
        corners.sort();

        let origin_tile_id = self.tiles[&corners[0]].id;
        let mut solved_tiles = HashMap::new();
        solved_tiles.insert(origin_tile_id, (0, 0));

        let mut neighbors = self.associations.remove(&origin_tile_id)?;
        neighbors.sort();
        for neighbor_id in neighbors.into_iter() {
            self.re_solve(&mut solved_tiles, origin_tile_id, neighbor_id)?;
        }

        let (min_x, min_y, max_x, max_y) = solved_tiles.values().fold((0, 0, 0, 0), |a, (x, y)| {
            (
                std::cmp::min(*x, a.0),
                std::cmp::min(*y, a.1),
                std::cmp::max(*x, a.2),
                std::cmp::max(*y, a.3),
            )
        });

        let mut sorted_tiles = solved_tiles.iter().collect::<Vec<_>>();
        sorted_tiles.sort_by(|(_, (x1, y1)), (_, (x2, y2))| {
            if y1 > y2 || (y1 == y2 && x1 < x2) {
                std::cmp::Ordering::Less
            } else if x1 == x2 && y1 == y2 {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        });

        Ok(TileSolution {
            tiles: sorted_tiles
                .iter()
                .map(|(id, _)| self.tiles.remove(*id).unwrap())
                .collect(),
            width: (max_x - min_x + 1) as usize,
            height: (max_y - min_y + 1) as usize,
        })
    }

    fn re_solve(
        &mut self,
        solved_tiles: &mut HashMap<usize, (i8, i8)>,
        solved_neighbor: usize,
        unsolved_id: usize,
    ) -> AOCResult<()> {
        if !solved_tiles.contains_key(&unsolved_id) {
            let transform_info =
                self.tiles[&unsolved_id].can_transform(&self.tiles[&solved_neighbor])?;

            let solved_position = solved_tiles[&solved_neighbor];
            let position = (
                transform_info.relative_position.0 + solved_position.0,
                transform_info.relative_position.1 + solved_position.1,
            );

            solved_tiles.insert(unsolved_id, position);
            self.tiles.get_mut(&unsolved_id)?.apply(&transform_info);

            let mut neighbors = self.associations.remove(&unsolved_id)?;
            neighbors.sort();
            for neighbor_id in neighbors.into_iter() {
                self.re_solve(solved_tiles, unsolved_id, neighbor_id)?;
            }
        }

        Ok(())
    }

    fn corners(&self) -> Vec<usize> {
        self.associations
            .iter()
            .filter_map(|(tile, associations)| {
                if associations.len() <= 2 {
                    Some(*tile)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn load_tiles(filename: &str) -> HashMap<usize, Tile> {
        let mut tiles = HashMap::new();
        let mut input = utils::get_input(filename);
        while let Ok(tile) = Tile::load(&mut input) {
            tiles.insert(tile.id, tile);
        }
        tiles
    }

    fn tile_associations<'a>(tiles: &HashMap<usize, Tile>) -> HashMap<usize, Vec<usize>> {
        let tiles = tiles.values().collect::<Vec<_>>();
        let mut tile_matches = HashMap::new();
        for i in 0..tiles.len() {
            let this_tile = tiles[i];
            for j in i + 1..tiles.len() {
                let that_tile = tiles[j];
                if this_tile.can_transform(that_tile).is_some() {
                    tile_matches
                        .entry(this_tile.id)
                        .or_insert(Vec::new())
                        .push(that_tile.id);
                    tile_matches
                        .entry(that_tile.id)
                        .or_insert(Vec::new())
                        .push(this_tile.id);
                }
            }
        }
        tile_matches
    }
}

impl TileSolution {
    pub fn image(&self, fullsize: bool) -> TileImage {
        let mut image_data = Vec::new();
        let num_pixels = if fullsize { 10 } else { 8 };
        let pixel_offset = if fullsize { 0 } else { 1 };
        for (i, tile) in self.tiles.iter().enumerate() {
            for y in 0..num_pixels {
                let image_y = y + (i / self.width) * num_pixels;
                let mut pixels = tile.image_content[y + pixel_offset]
                    .get(pixel_offset..10 - pixel_offset)
                    .unwrap()
                    .iter()
                    .map(|v| *v)
                    .collect::<Vec<_>>();
                if image_y == image_data.len() {
                    image_data.push(pixels);
                } else {
                    image_data[image_y].append(&mut pixels);
                }
            }
        }
        TileImage { pixels: image_data }
    }
}

use std::fmt;
impl fmt::Display for TileImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.pixels.iter() {
            write!(
                f,
                "{}\n",
                row.iter()
                    .map(|v| if *v { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl TileImage {
    pub fn rotate_clockwise(&mut self) {
        let old_image = self.pixels.clone();
        let num_rows = old_image.len();
        for (i, row) in self.pixels.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = old_image[num_rows - 1 - j][i];
            }
        }
    }

    pub fn flip_vertical(&mut self) {
        let num_rows = self.pixels.len();
        for i in 0..num_rows / 2 {
            for j in 0..self.pixels[i].len() {
                let old_pixel = self.pixels[i][j];
                self.pixels[i][j] = self.pixels[num_rows - 1 - i][j];
                self.pixels[num_rows - 1 - i][j] = old_pixel;
            }
        }
    }

    pub fn part2(&mut self) -> usize {
        for do_flip in [false, true].iter() {
            if *do_flip {
                self.flip_vertical();
            }

            for _ in 0..4 {
                if self.delete_monsters() {
                    return self.count_markers();
                }
                self.rotate_clockwise();
            }
        };

        return 0;
    }

    pub fn delete_monsters(&mut self) -> bool {
        let mut monster_check = Vec::new();
        "                  # ".chars().enumerate().fold((), |_, (i,c)| if c == '#' {monster_check.push((0,i))} );
        "#    ##    ##    ###".chars().enumerate().fold((), |_, (i,c)| if c == '#' {monster_check.push((1,i))} );
        " #  #  #  #  #  #   ".chars().enumerate().fold((), |_, (i,c)| if c == '#' {monster_check.push((2,i))} );

        let mut found_monster = false;
        for i in 0..self.pixels.len()-2 {
            for j in 0..self.pixels[i].len()-20 {
                if monster_check.iter().all(|delta| self.pixels[i+delta.0][j+delta.1]) {
                    found_monster = true;
                    for delta in monster_check.iter() {
                        self.pixels[i+delta.0][j+delta.1] = false;
                    }
                }
            }
        }

        found_monster
    }

    pub fn count_markers(&self) -> usize {
        self.pixels.iter().fold(0, |a, row| row.iter().fold(a, |count, pixel| count + if *pixel {1} else {0}))
    }
}

pub fn part1(puzzle: &TilePuzzle) -> u64 {
    puzzle.corners().iter().fold(1, |a, id| a * (*id as u64))
}

pub fn day20() {
    let puzzle = TilePuzzle::load("day20").unwrap();
    println!("jurassic_jigsaw part 1: {:?}", part1(&puzzle));

    let solution = puzzle.solve().unwrap();
    println!("jurassic_jigsaw part 2: {:?}", solution.image(false).part2());
}

#[cfg(test)]
mod tests {
    use crate::day20::*;
    #[test]
    pub fn test_day20_aoc() {
        let puzzle = TilePuzzle::load("test_day20").unwrap();
        assert_eq!(part1(&puzzle), 20899048083289);

        let solution = puzzle.solve().unwrap();

        println!("{}", solution.image(false));

        assert_eq!(solution.image(false).part2(), 273);
    }

    /*pub fn test_day20_rotate() {
        let tile_input = vec![
            ".##.###..#".to_owned(),
            "#......##.".to_owned(),
            "..#.#.....".to_owned(),
            "#...#..#.#".to_owned(),
            "#..#....##".to_owned(),
            "..##.##.##".to_owned(),
            ".#.#..####".to_owned(),
            "#.....##..".to_owned(),
            "..#.......".to_owned(),
            "..###..##.".to_owned(),
        ];

        let mut tile = Tile::load(&mut tile_input.into_iter()).unwrap();
        let transform = TileTransform {
            flipped_edge: None,
            rotate_steps: 1,
            relative_position: (0, 1),
        };

        tile.apply(&transform);
        let image = TileSolution {
            tiles: vec![tile],
            width: 1,
            height: 1,
        };

        assert_eq!(
            image.image(true),
            vec![
                "..#..##.#.",
                "...#.....#",
                "##..#..#.#",
                "#..###....",
                "#.....##.#",
                "....#....#",
                "..###....#",
                "#.##..#.#.",
                "#..###..#.",
                "...####..#",
            ]
        );
    }*/

    /*#[test]
    pub fn test_day20_basic() {
        let mut puzzle = TilePuzzle::load("test_day20_basic").unwrap();

        let transform = puzzle.tiles[&2].can_transform(&puzzle.tiles[&1]).unwrap();
        assert_eq!(transform.rotate_steps, 3);
        assert_eq!(transform.relative_position, (1, 0));
        assert_eq!(transform.flipped_edge, None);

        puzzle.tiles.get_mut(&2).unwrap().apply(&transform);
        let transformed_image = TileSolution {
            width: 1,
            height: 1,
            tiles: vec![puzzle.tiles[&2].clone()],
        };
        assert_eq!(
            transformed_image.image(true),
            vec![
                "#.........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "#########.",
                "..........",
            ]
        );
        assert_eq!(
            transformed_image.image(false),
            vec![
                "........", "........", "........", "........", "........", "........", "........",
                "########",
            ]
        );

        let solution = puzzle.solve().unwrap();
        assert_eq!(
            solution.image(true),
            vec![
                "###########.........",
                ".#..................",
                "....................",
                "....................",
                "....................",
                "#...................",
                "....................",
                "....................",
                ".##......##########.",
                "....#...............",
            ]
        );
        assert_eq!(
            solution.image(false),
            vec![
                "#...............",
                "................",
                "................",
                "................",
                "................",
                "................",
                "................",
                "##......########"
            ]
        );
    }*/
}
