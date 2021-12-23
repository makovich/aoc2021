use std::collections::HashSet;

pub fn main() {
    let (points, instrs) = include_str!("./day13.input").split_once("\n\n").unwrap();
    let (axis, pos) = instrs
        .lines()
        .nth(0)
        .map(|l| l.split_once('=').unwrap())
        .map(|(axis, pos)| (axis.chars().last().unwrap(), pos.parse::<u32>().unwrap()))
        .unwrap();

    // println!("{:?}", points);

    let res = points
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .fold(HashSet::<(u32, u32)>::new(), |mut s, (x, y)| {
            match axis {
                'y' => {
                    s.insert((x, if y > pos { pos - (y - pos) } else { y }));
                }
                'x' => {
                    s.insert((if x > pos { pos - (x - pos) } else { x }, y));
                }
                _ => unreachable!(),
            }
            s
        });

    println!("{:?}", res.len());
}
