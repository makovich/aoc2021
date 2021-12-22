use std::cmp::{max, min};

pub fn main() {
    let mut dots = include_str!("./day5.input")
        .lines()
        .map(
            |v| match v.split(&[',', ' '][..]).collect::<Vec<_>>().as_slice() {
                &[x1, x2, _, y1, y2] => (
                    x1.parse().unwrap(),
                    x2.parse().unwrap(),
                    y1.parse().unwrap(),
                    y2.parse().unwrap(),
                ),
                _ => unreachable!(),
            },
        )
        .map(|(x1, x2, y1, y2)| {
            (
                min::<i32>(x1, x2),
                min::<i32>(y1, y2),
                max::<i32>(x1, x2),
                max::<i32>(y1, y2),
            )
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .flat_map(|(x1, x2, y1, y2)| {
            if x1 == x2 {
                (y1..=y2).map(|y| (x1, y)).collect::<Vec<_>>()
            } else {
                (x1..=x2).map(|x| (x, y1)).collect::<Vec<_>>()
            }
        })
        .map(|(x, y)| x * 1000 + y)
        // println!("{:?}", r);
        // 1
        // .collect::<Vec<(i32, i32)>>();
        .collect::<Vec<i32>>();

    dots.sort();
    // println!("{:?}", dots);

    let mut res = 0;
    let mut skip = false;
    let mut cur = -1;
    let mut cnt = 0;
    for x in dots {
        if x != cur {
            skip = false;
        }
        if skip {
            continue;
        }
        if x == cur {
            cnt += 1;
        } else {
            cur = x;
            cnt = 1;
        }
        if cnt > 1 {
            res += 1;
            skip = true;
        }
    }

    println!("{:?}", res);
}
