use std::collections::HashSet;

pub fn main() {
    let (points, instrs) = include_str!("./day13.input").split_once("\n\n").unwrap();
    let instrs = instrs
        .lines()
        .map(|l| l.split_once('=').unwrap())
        .map(|(axis, pos)| (axis.chars().last().unwrap(), pos.parse::<u32>().unwrap()))
        .collect::<Vec<(char, u32)>>();

    println!("{:?}", instrs);

    let points = points
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<HashSet<(u32, u32)>>();

    println!("{:?}", points);

    let res = instrs.iter().fold(points, |set, (axis, pos)| {
        set.iter().fold(HashSet::new(), |mut s, (x, y)| {
            match axis {
                'y' => {
                    s.insert((*x, if y > pos { pos - (y - pos) } else { *y }));
                }
                'x' => {
                    s.insert((if x > pos { pos - (x - pos) } else { *x }, *y));
                }
                _ => unreachable!(),
            }
            s
        })
    });

    println!("{:?}", res);

    let dim = instrs
        .iter()
        .fold((0, 0), |(w, h), (axis, pos)| match axis {
            'x' => (*pos as usize, h as usize),
            'y' => (w as usize, *pos as usize),
            _ => unreachable!(),
        });

    println!("{:?}", dim);

    let mut paper = vec![vec![' '; dim.0]; dim.1];

    res.iter().for_each(|(x, y)| {
        paper[*y as usize][*x as usize] = '#';
    });

    for line in paper {
        for x in line {
            print!("{}", x);
        }
        println!();
    }
}
