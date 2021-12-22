pub fn main() {
    let r = include_str!("./day8.input")
        .lines()
        .flat_map(|l| {
            l.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .filter(|d| [2, 3, 4, 7].iter().any(|&x| x == d.len()))
        })
        .count();

    println!("{:?}", r);
}
