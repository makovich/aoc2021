#[derive(Debug)]
enum Cmd {
    F(u64),
    U(u64),
    D(u64),
}

impl From<&str> for Cmd {
    fn from(s: &str) -> Cmd {
        match s.split(' ').collect::<Vec<&str>>()[..] {
            ["forward", v] => Cmd::F(v.parse::<u64>().unwrap()),
            ["down", v] => Cmd::D(v.parse::<u64>().unwrap()),
            ["up", v] => Cmd::U(v.parse::<u64>().unwrap()),
            _ => panic!("unknown command"),
        }
    }
}

pub fn main() {
    println!(
        "{}",
        include_str!("./day2.input")
            .lines()
            .map(Into::into)
            .fold((0, 0, 0), |mut acc, val| {
                match val {
                    Cmd::F(v) => acc.0 += v,
                    Cmd::D(v) => acc.1 += v,
                    Cmd::U(v) => acc.1 -= v,
                };
                acc.2 = acc.0 * acc.1;
                acc
            })
            .2
    );
}
