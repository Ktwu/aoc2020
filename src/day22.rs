#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCResult, AOCError};
use std::collections::HashSet;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct CombatGame {
    p1: Vec<usize>,
    p2: Vec<usize>,
    cache: HashSet<u64>,
    recurse: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2
}

impl CombatGame {
    pub fn subgame(&self, p1: usize, p2: usize) -> CombatGame {
        CombatGame {
            p1: self.p1[..p1].to_vec(),
            p2: self.p2[..p2].to_vec(),
            cache: HashSet::new(),
            recurse: true,
        }
    }

    pub fn load(filename: &str, recurse: bool) -> AOCResult<CombatGame> {
        let mut game = CombatGame {
            p1: Vec::new(),
            p2: Vec::new(),
            cache: HashSet::new(),
            recurse,
        };

        let mut player_ref = &mut game.p1;
        let input = utils::get_input(filename);
        for line in input {
            match line.as_str() {
                "Player 1:" => {
                    player_ref = &mut game.p1;
                },
                "Player 2:" => {
                    player_ref = &mut game.p2;
                },
                "" => {},
                _ => {
                    player_ref.push(line.parse::<usize>()?)
                }
            }
        }

        Ok(game)
    }

    pub fn step(&mut self) -> Option<Player> {
        if self.p1.len() == 0 {
            return Some(Player::P2);
        } else if self.p2.len() == 0 {
            return Some(Player::P1);
        }

        let mut s = DefaultHasher::new();
        self.p1.hash(&mut s);
        self.p2.hash(&mut s);
        let round_hash = s.finish();
        if self.cache.contains(&round_hash) {
            return Some(Player::P1)
        }
        self.cache.insert(round_hash);

        let card1 = self.p1.remove(0);
        let card2 = self.p2.remove(0);

        match if self.recurse && self.p1.len() >= card1 && self.p2.len() >= card2 {
            self.subgame(card1, card2).play().0
        } else if card1 > card2 {
            Player::P1
        } else {
            Player::P2
        } {
            Player::P1 => {self.p1.push(card1); self.p1.push(card2);},
            Player::P2 => {self.p2.push(card2); self.p2.push(card1);},
        };

        None
    }

    fn score(&self, player: Player) -> usize {
        let winner = match player {
            Player::P1 => &self.p1,
            Player::P2 => &self.p2,
        };
        winner.iter().enumerate().fold(0, |a, (i,card)| a + card * (winner.len()-i))
    }

    pub fn play(&mut self) -> (Player, usize) {
        loop {
            match self.step() {
                Some(s) => {return (s, self.score(s));},
                None => ()
            }
        }
    }
}

pub fn day22() {
    let mut game = CombatGame::load("day22", false).unwrap();
    println!("crab_combat part 1: {:?}", game.play());

    let mut game = CombatGame::load("day22", true).unwrap();
    println!("crab_combat part 2: {:?}", game.play());
}

#[cfg(test)]
mod tests {
    use crate::day22::*;

    #[test]
    pub fn test_day22() {
        let mut game = CombatGame::load("test_day22", false).unwrap();
        assert_eq!(
            game.play(),
            (Player::P2, 306),
        );

        let mut game = CombatGame::load("test_day22", true).unwrap();
        assert_eq!(
            game.play(),
            (Player::P2, 291)
        );
    }
}