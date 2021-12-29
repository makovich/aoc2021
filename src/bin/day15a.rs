use std::collections::{HashSet, VecDeque};

const SIZE: usize = 100;
const NEIB: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
struct Cell {
    weig: u16,
    dist: u16,
}

pub fn main() {
    let mut cave = include_str!("./day15.input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell {
                    weig: c.to_digit(10).unwrap() as u16,
                    dist: u16::MAX,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // eprintln!("{:?}", cave);

    cave[0][0].dist = 0;

    let mut dist = 0;
    let mut seen = HashSet::new();
    let mut q = VecDeque::from([(0, 0)]);

    for _ in 0..SIZE * SIZE {
        seen.clear();

        while let Some((row, col)) = q.pop_front() {
            if !seen.insert((row, col)) {
                continue;
            }

            dist = cave[row][col].dist;

            NEIB.iter().for_each(|(dx, dy)| {
                let neib_row = (row as isize + dy) as usize;
                let neib_col = (col as isize + dx) as usize;

                if let Some(neib) = cave
                    .get_mut(neib_row)
                    .map(|r| r.get_mut(neib_col))
                    .flatten()
                {
                    neib.dist = u16::min(neib.dist, dist + neib.weig);
                    q.push_back((neib_row, neib_col));
                }
            });
        }
    }

    println!("{}", dist);
}
