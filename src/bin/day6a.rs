pub fn main() {
    let res = include_str!("./day6.test")
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut acc, num| {
            acc[num] += 1;
            acc
        });

    let res = (0..256).fold(res, |mut days, _| {
        let x = days[0];
        days.rotate_left(1);
        days[6] += x;
        days
    });

    println!("{:?}", res.iter().sum::<u64>());
}
