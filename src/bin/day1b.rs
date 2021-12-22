pub fn main() {
    let mms = include_str!("day1.input")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let r = mms
        .windows(4)
        .map(|v| (v[0] + v[1] + v[2], v[1] + v[2] + v[3]))
        .fold(0, |acc, (prv, cur)| acc + if prv < cur { 1 } else { 0 });

    println!("{}", r);

    // println!("hello {}", env!("CARGO_BIN_NAME"));
    // println!("{}", module_path!());
}
