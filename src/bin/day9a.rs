const W: usize = 100;
const H: usize = 100;

pub fn main() {
    let hmap = include_str!("./day9.input")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .fold([[0u32; W]; H], |mut sum, (idx, c)| {
            let x = idx % W;
            let y = idx / W;

            sum[y][x] = c;

            sum
        });

    let mut res = 0;

    for (y, line) in hmap.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let t = hmap.get(y - 1).and_then(|line| line.get(x));
            let r = line.get(x + 1);
            let b = hmap.get(y + 1).and_then(|line| line.get(x));
            let l = line.get(x - 1);
            if [t, r, b, l].iter().filter_map(|&v| v).all(|v| v > c) {
                res += c + 1;
            }
        }
    }

    println!("{:?}", res);
}
