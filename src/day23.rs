#![allow(dead_code)]

pub struct CrabCups<const N: usize> {
    cups: Vec<usize>,
    current: usize,
    steps: usize,
}

impl <const N: usize> CrabCups <{N}> {
    pub fn load(labels: &[usize]) -> CrabCups<{N}> {
        let mut cups = vec![0; N];
        for i in 0..N {
            let cup_index = if i < labels.len() {
                labels[i] - 1
            } else {
                i
            };
            let next_cup = if (i+1)%N < labels.len() {
                labels[(i+1)%N] - 1
            } else {
                (i + 1) % N
            };         
            cups[cup_index] = next_cup;
        }

        CrabCups {
            current: labels[0] - 1,
            steps: 0,
            cups,
        }
    }

    pub fn step(&mut self) {
        let r1 = self.cups[self.current];
        let r2 = self.cups[r1];
        let r3 = self.cups[r2];
        self.cups[self.current] = self.cups[r3];
        for r in &[r1, r2, r3] { self.cups[*r] = N }

        let mut destination = self.current;
        destination = loop {
            destination = (destination + N - 1) % N;
            if self.cups[destination] != N {
                break destination;
            }
        };

        let cup = self.cups[destination];
        self.cups[destination] = r1;
        self.cups[r1] = r2;
        self.cups[r2] = r3;
        self.cups[r3] = cup;

        self.current = self.cups[self.current];
        self.steps += 1;
    }

    fn to_labels(&self) -> Vec<usize> {
        let mut cups = self.cups.clone();
        let mut next_index = self.steps % N;
        let mut next_cup = self.current;
        for _ in 0..N {
            cups[next_index] = next_cup + 1;
            next_cup = self.cups[next_cup];
            next_index = (next_index + 1) % N
        }
        cups
    }

    pub fn into_iter(mut self) -> impl Iterator<Item = Vec<usize>> {
        std::iter::from_fn(move || {
            self.step();
            Some(self.to_labels())
        })
    }
}

pub fn day23() {
    let input = [2,1,9,3,4,7,8,6,5];
    let cups: CrabCups<9> = CrabCups::load(&input);
    println!("crab_cups part 1: {:?}", cups.into_iter().nth(99));

    let mut cups: CrabCups<1000000> = CrabCups::load(&input);
    for _ in 0..10000000 {
        cups.step();
    }
    let i1 = cups.cups[0] as u64;
    let i2 = cups.cups[i1 as usize] as u64;

    println!("crab_cups part 2: {:?}", (i1+1) * (i2+1));

}

#[cfg(test)]
mod tests {
    use crate::day23::*;

    #[test]
    pub fn test_day23() {       
        let mut cups: CrabCups<9> = CrabCups::load(&[3,8,9,1,2,5,4,6,7]);
        assert_eq!(
            cups.cups,
            vec![1,4,7,5,3,6,2,8,0]
        );

        cups.step();
        assert_eq!(
            cups.cups,
            vec![4,7,1,5,3,6,2,8,0]
        );
        assert_eq!(
            cups.current,
            1
        );

        assert_eq!(
            cups.to_labels(),
            vec![3,2,8,9,1,5,4,6,7]
        );

        let mut i = cups.into_iter();
        assert_eq!(
            i.nth(0),
            Some(vec![3,2,5,4,6,7,8,9,1])
        );

        assert_eq!(
            i.nth(7),
            Some(vec![5,8,3,7,4,1,9,2,6])
        );
    }

    #[test]
    pub fn test_day23_big() {
        let mut cups: CrabCups<1000000> = CrabCups::load(&[3,8,9,1,2,5,4,6,7]);
        assert_eq!(
            cups.cups[0..11],
            [1,4,7,5,3,6,9,8,0,10,11],
        );

        for _ in 0..10000000 {
            cups.step();
        }
        assert_eq!(
            cups.cups[0], 934000
        );
        assert_eq!(
            cups.cups[934000], 159791
        );

    }
}
