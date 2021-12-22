pub fn main() {
    let mut a = include_str!("./day7.input")
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();

    a.sort();

    let mid = a[a.len() / 2];

    dbg!(a.iter().map(|v| (v - mid).abs()).sum::<i32>());
}
