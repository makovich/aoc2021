pub fn main() {
    let r = include_str!("./day8.input")
        .lines()
        .map(|l| {
            let (samples, display) = l.split_once(" | ").unwrap();

            let make_mask = |digit: &str| {
                digit
                    .as_bytes()
                    .iter()
                    .fold(0, |acc, dig| acc | (1 << (dig % 7)))
            };

            let [one, four] = samples.split(' ').fold([0u8; 2], |mut res, dig| {
                match dig.len() {
                    2 => res[0] = make_mask(dig),
                    4 => res[1] = make_mask(dig),
                    _ => {}
                }
                res
            });

            display
                .split(' ')
                .rev()
                .enumerate()
                .map(|(idx, dig)| (10u32.pow(idx as u32), make_mask(dig)))
                .map(|(base_ten, mask)| {
                    base_ten
                        * match mask.count_ones() {
                            2 => 1,
                            4 => 4,
                            3 => 7,
                            7 => 8,
                            5 if (mask ^ one).count_ones() == 3 => 3,
                            5 if (mask ^ four).count_ones() == 5 => 2,
                            5 => 5,
                            6 if (mask ^ one).count_ones() == 6 => 6,
                            6 if (mask ^ four).count_ones() == 2 => 9,
                            6 => 0,
                            _ => 0,
                        }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{:?}", r);
}
