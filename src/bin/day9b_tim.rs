const NEXT: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub fn main() {
    let mut input = include_bytes!("./day9.test")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<u8>>>();

    let mut basins = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            (input[y][x] < b'9').then(|| basins.push(basin(&mut input, x, y)));
            for line in &input {
                println!("{}", unsafe { String::from_utf8_unchecked(line.clone()) });
            }
            println!("-----");
        }
    }

    basins.sort_unstable();
    println!("{}", basins.iter().rev().take(3).product::<usize>());
}

fn basin(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    map[y][x] = b'9';
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get(y).and_then(|l| l.get(x)).map(|&n| n < b'9') {
                Some(true) => acc + basin(map, x, y),
                _ => acc,
            }
        })
}
