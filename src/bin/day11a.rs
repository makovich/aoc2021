const W: usize = 10;
const H: usize = 10;
const NEIB: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn main() {
    let mut map = [[0i8; W]; H];

    include_str!("./day11.input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .for_each(|(idx, val)| {
            let x = idx % W;
            let y = idx / W;

            map[y][x] = val as i8;
        });

    println!("{:?}", map);

    let x = std::iter::repeat(())
        .take(100)
        .inspect(|_| println!())
        .map(|_| {
            (0..W * H).map(|i| (i % W, i / H)).for_each(|(x, y)| {
                map[y][x] += 1;

                if map[y][x] == 10 {
                    let _ = NEIB.iter().fold(0, |_acc, (dx, dy)| {
                        visit(&mut map, (x as i8 + dx) as usize, (y as i8 + dy) as usize)
                    });
                }
            });

            println!("\ncounting");

            (0..W * H).map(|i| (i % W, i / H)).fold(0, |acc, (x, y)| {
                if map[y][x] >= 10 {
                    map[y][x] = 0;
                    if x % 10 == 0 {
                        println!();
                    }

                    print!("{:3}", map[y][x]);
                    acc + 1
                } else {
                    if x % 10 == 0 {
                        println!();
                    }

                    print!("{:3}", map[y][x]);
                    acc
                }
            })
        })
        .inspect(|s| println!("sum={}", s))
        .sum::<u32>();

    println!("\n\n{}", x);
}

fn visit(map: &mut [[i8; W]; H], x: usize, y: usize) -> u32 {
    // println!("x={} y={}", x, y);
    if map.get(y).map(|r| r.get(x)).flatten().is_none() {
        return 0;
    }

    if map[y][x] >= 10 {
        return 0;
    }

    map[y][x] += 1;

    if map[y][x] < 10 {
        return 0;
    }

    1 + NEIB.iter().fold(0, |acc, (dx, dy)| {
        acc + visit(map, (x as i8 + dx) as usize, (y as i8 + dy) as usize)
    })
}
