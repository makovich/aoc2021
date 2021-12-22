pub fn main() {
    let (h, d, _) = include_str!("./day2.input")
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .fold((0, 0, 0), |(h, d, a), (c, v)| {
            match (c, v.parse::<i32>().unwrap()) {
                ("forward", v) => (h + v, d + a * v, a),
                ("down", v) => (h, d, a + v),
                ("up", v) => (h, d, a - v),
                _ => unreachable!(),
            }
        });

    println!("{}", h * d)
}
