use std::cmp::{max, min};

pub fn main() {
    let mut dots = include_str!("./day5.input")
        .lines()
        .flat_map(|line| {
            line.split_once(" -> ")
                .into_iter()
                .flat_map(|(a, b)| {
                    let (x1, y1) = a
                        .split_once(',')
                        .map(|(x1, y1)| (x1.parse().unwrap(), y1.parse().unwrap()))
                        .unwrap();

                    let (x2, y2) = b
                        .split_once(',')
                        .map(|(x2, y2)| (x2.parse().unwrap(), y2.parse().unwrap()))
                        .unwrap();

                    if x1 == x2 {
                        (min(y1, y2)..=max(y1, y2)).map(|y: i32| (x1, y)).collect()
                    } else if y1 == y2 {
                        (min(x1, x2)..=max(x1, x2)).map(|x: i32| (x, y1)).collect()
                    } else {
                        vec![]
                    }
                })
                .map(|(x, y)| x * 1000 + y)

            // println!("{:?}", r);

            // 1
        })
        // .collect::<Vec<(i32, i32)>>();
        .collect::<Vec<_>>();

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
