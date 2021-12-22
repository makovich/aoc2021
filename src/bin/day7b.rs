pub fn main() {
    let a = include_str!("./day7.input")
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();

    let mean = a.iter().sum::<i32>() / a.len() as i32;

    dbg!(mean);

    let mut min = i32::max_value();

    for k in (mean - 4..=mean + 4) {
        let s = dbg!(a
            .iter()
            .map(|v| (((k - v).abs() + 1) * (k - v).abs()) / 2)
            .sum::<i32>());

        min = std::cmp::min(s, min);
    }

    println!("{:?}", min);
}
