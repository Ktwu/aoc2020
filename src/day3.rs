use crate::{utils, utils::AOCError};

pub struct Trajectory {
    right: usize,
    down: usize,
    tree_count: u64,
    right_offset: usize,
    to_skip: usize,
}

impl Trajectory {
    fn new(right: usize, down: usize) -> Self {
        Trajectory {
            right,
            down,
            tree_count: 0,
            right_offset: 0,
            to_skip: 0,
        }
    }

    fn step(&mut self, line: &str) -> Result<(), AOCError> {
        if self.to_skip == 0 {
            let target = line.chars().nth(self.right_offset % line.len())?;
            if target == '#' {
                self.tree_count += 1;
            }
            self.right_offset = (self.right_offset + self.right) % line.len();
            self.to_skip = self.down;
        }
        self.to_skip -= 1;
        Ok(())
    }

    fn count(&self) -> u64 {
        self.tree_count
    }
}

pub fn day3() {
    let input = utils::get_input("day3");
    println!(
        "toboggan trajectory part 1: {}",
        toboggan_trajectory_p1(input).unwrap()
    );

    let input = utils::get_input("day3");
    println!(
        "toboggan trajectory part 2: {}",
        toboggan_trajectory_p2(input).unwrap()
    );
}

pub fn toboggan_trajectory<T: Iterator<Item = String>>(
    input: T,
    mut trajectory: Trajectory,
) -> Result<u64, AOCError> {
    for line in input {
        trajectory.step(&line)?;
    }
    Ok(trajectory.count())
}

pub fn toboggan_trajectory_p1<T: Iterator<Item = String>>(input: T) -> Result<u64, AOCError> {
    toboggan_trajectory(input, Trajectory::new(3, 1))
}

pub fn toboggan_trajectory_p2<T: Iterator<Item = String>>(input: T) -> Result<u64, AOCError> {
    let mut trajectories = vec![
        Trajectory::new(1, 1),
        Trajectory::new(3, 1),
        Trajectory::new(5, 1),
        Trajectory::new(7, 1),
        Trajectory::new(1, 2),
    ];

    for line in input {
        for trajectory in trajectories.iter_mut() {
            trajectory.step(&line)?;
        }
    }

    Ok(trajectories
        .iter()
        .fold(1, |a: u64, t: &Trajectory| a * t.count() as u64))
}

#[test]
fn basic_toboggan_trajectory() {
    let test_input = utils::get_input("test_day3");
    assert_eq!(toboggan_trajectory_p1(test_input).unwrap(), 7);

    let test_input = utils::get_input("test_day3");
    assert_eq!(
        toboggan_trajectory(test_input, Trajectory::new(1, 2)).unwrap(),
        2
    );

    let test_input = utils::get_input("test_day3");
    assert_eq!(
        toboggan_trajectory(test_input, Trajectory::new(7, 1)).unwrap(),
        4
    );

    let test_input = utils::get_input("test_day3");
    assert_eq!(toboggan_trajectory_p2(test_input).unwrap(), 336);
}
