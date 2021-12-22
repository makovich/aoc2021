// const WIDTH: usize = 5;
// const LINES: i32 = 12;
const WIDTH: usize = 12;
use std::{cmp::Ordering, fmt};

struct V<'a>(&'a Vec<i32>);

impl fmt::Binary for V<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (count, n) in self.0.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:b}", n)?;
        }
        Ok(())
    }
}
pub fn main() {
    // let log = include_str!("./day3.input")
    let log = include_str!("./day3.input")
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = dbg!(find_bits(log.clone(), true));
    let co2 = dbg!(find_bits(log, false));
    dbg!(oxy * co2);
}

fn find_bits(mut log: Vec<i32>, most_win: bool) -> i32 {
    for i in (0..WIDTH).rev() {
        let mut v0 = vec![];
        let mut v1 = vec![];

        for n in &log {
            let bit = (n & (1 << i)) >> i;
            if bit == 0 {
                v0.push(*n);
            } else {
                v1.push(*n);
            }
        }

        log = match v0.len().partial_cmp(&v1.len()).unwrap() {
            Ordering::Equal if most_win => v1,
            Ordering::Equal => v0,
            Ordering::Less if most_win => v1,
            Ordering::Less => v0,
            Ordering::Greater if most_win => v0,
            Ordering::Greater => v1,
        };

        if log.len() == 1 {
            break;
        }
    }

    log[0]
}
