use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

const SIZE: usize = 100;
const NEIB: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
struct Cell {
    mark: bool,
    weig: u16,
    dist: u16,
}

#[derive(Eq, Ord)]
struct Nxt {
    cell: (usize, usize),
    dist: u16,
}

impl Nxt {
    fn new(row: usize, col: usize, dist: u16) -> Reverse<Nxt> {
        Reverse(Self {
            cell: (row, col),
            dist,
        })
    }
}

impl PartialEq for Nxt {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for Nxt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

pub fn main() {
    let mut cave = include_str!("./day15.input")
        .lines()
        .cycle()
        .take(SIZE * 5)
        .enumerate()
        .map(|(dy, line)| {
            line.chars()
                .cycle()
                .take(SIZE * 5)
                .enumerate()
                .map(|(dx, chr)| Cell {
                    mark: false,
                    weig: chr
                        .to_digit(10)
                        .map(|w| (w as usize + dy / SIZE + dx / SIZE - 1) % 9 + 1)
                        .unwrap() as u16,
                    dist: u16::MAX,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    cave[0][0].dist = 0;

    let mut dist = 0;
    let mut q = BinaryHeap::from([Nxt::new(0, 0, 0)]);

    while let Some(Reverse(Nxt { cell, dist: _ })) = q.pop() {
        let (row, col) = cell;

        if cave[row][col].mark {
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
                q.push(Nxt::new(neib_row, neib_col, neib.dist));
            }
        });

        cave[row][col].mark = true;
    }

    println!("{}", dist);
}
