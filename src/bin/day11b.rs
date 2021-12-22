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

    std::iter::successors(Some(1u32), |&v| v.checked_add(1))
        .map(|step| {
            (0..W * H).map(|i| (i % W, i / H)).for_each(|(x, y)| {
                map[y][x] += 1;

                (map[y][x] == 10).then(|| {
                    NEIB.iter().for_each(|(dx, dy)| {
                        visit(&mut map, (x as i8 + dx) as usize, (y as i8 + dy) as usize)
                    })
                });
            });

            let flashes = (0..W * H).map(|i| (i % W, i / H)).fold(0, |acc, (x, y)| {
                let flash = if map[y][x] >= 10 {
                    map[y][x] = 0;
                    1
                } else {
                    0
                };

                if x % 10 == 0 {
                    eprintln!();
                }
                eprint!("{:3}", map[y][x]);

                acc + flash
            });

            (step, flashes)
        })
        .inspect(|(s, f)| eprintln!("\nstep={} flashes={}", s, f))
        .take_while(|(_, f)| *f != 100)
        .last()
        .map(|(s, _)| println!("{}", s + 1));
}

fn visit(map: &mut [[i8; W]; H], x: usize, y: usize) {
    if map.get(y).map(|r| r.get(x)).flatten().is_none() {
        return;
    }

    if map[y][x] >= 10 {
        return;
    }

    map[y][x] += 1;

    if map[y][x] < 10 {
        return;
    }

    NEIB.iter()
        .for_each(|(dx, dy)| visit(map, (x as i8 + dx) as usize, (y as i8 + dy) as usize))
}
