pub fn main() {
    let mms = include_str!("day1.input")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u16>>();

    let r = mms
        .windows(2)
        .map(|v| (v[0], v[1]))
        .fold(0, |acc, (prv, cur)| acc + if prv < cur { 1 } else { 0 });

    println!("{}", r);
    // println!("hello {}", env!("CARGO_BIN_NAME"));
    // println!("{}", module_path!());
}
