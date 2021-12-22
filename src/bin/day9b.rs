use std::collections::BinaryHeap;

const W: usize = 100;
const H: usize = 100;

pub fn main() {
    let mut set = [[false; W]; H];

    let heimap = include_str!("./day9.input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .fold([[0u32; W]; H], |mut sum, (idx, hei)| {
            let col = idx % W;
            let row = idx / W;
            sum[row][col] = hei;
            sum
        });

    let mut centers = vec![];

    let mut res = BinaryHeap::new();

    for (row, line) in heimap.iter().enumerate() {
        for (col, hei) in line.iter().enumerate() {
            let neibs = neibs(&heimap, col, row);
            if neibs.iter().flatten().all(|(v, _, _)| v > hei) {
                set[row][col] = true;

                let mut siz = 1;
                neibs.iter().flatten().for_each(|(v, r, c)| {
                    println!("val={} row={} col={}", v, r, c);
                    siz += visit(&heimap, &mut set, *v, *r, *c)
                });

                centers.push((siz, col, row));
                res.push(siz);
            }
        }
    }

    centers.sort_unstable();
    println!(
        "{:?}",
        centers.iter().rev().take(3).map(|v| v.0).product::<u32>()
    );
    println!(
        "{:?}",
        [res.pop(), res.pop(), res.pop()]
            .iter()
            .flatten()
            .product::<u32>()
    );
}

fn visit(
    heimap: &[[u32; W]; H],
    set: &mut [[bool; W]; H],
    val: u32,
    row: usize,
    col: usize,
) -> u32 {
    if val == 9 || set[row][col] {
        return 0;
    }

    set[row][col] = true;

    let mut siz = 1;

    neibs(&heimap, col, row)
        .iter()
        .flatten()
        .for_each(|(v, r, c)| {
            println!("> val={} row={} col={}", v, r, c);
            siz += visit(&heimap, set, *v, *r, *c)
        });

    siz
}

fn neibs(hmap: &[[u32; W]; H], col: usize, row: usize) -> [Option<(u32, usize, usize)>; 4] {
    let t = hmap
        .get(row - 1)
        .and_then(|line| line.get(col))
        .copied()
        .map(|val| (val, row - 1, col));

    let r = hmap
        .get(row)
        .and_then(|line| line.get(col + 1))
        .copied()
        .map(|val| (val, row, col + 1));

    let b = hmap
        .get(row + 1)
        .and_then(|line| line.get(col))
        .copied()
        .map(|val| (val, row + 1, col));

    let l = hmap
        .get(row)
        .and_then(|line| line.get(col - 1))
        .copied()
        .map(|val| (val, row, col - 1));

    [t, r, b, l]
}
