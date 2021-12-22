// #![feature(drain_filter)]

macro_rules! aoc2021_main {
    ( $( $day:tt ),+ $(,)? ) => {
        mod bin {
            $( pub mod $day; )+
        }

        fn main() {
            let mut tx = Vec::new();

        $(
            let t = std::time::Instant::now();
            println!(">> {}", stringify!($day));
            bin::$day::main();
            tx.push((stringify!($day), t.elapsed()));
        )+

            println!("\n\nTotals:");
            let mut sum = std::time::Duration::new(0,0);
            for (d, t) in tx {
                println!("{:>10}   {:?}", d, t);
                sum += t;
            }
            println!("            ------------");
            println!("             {:?}", sum);
        }
    };
}

aoc2021_main! {
    day1a,
    day1b,

    day2a,
    day2b,

    day3a,
    day3b,
    // day3c,

    day4a_v1,
    day4a_v2,
    day4a_tim,
    day4b,
    // day4b_tim,

    day5a,
    day5a_v2,
    day5a_tim,
    day5b,
    day5b_v2,
    day5b_tim,

    day6a,
    day6a_tim,

    day7a,
    day7b,

    day8a,
    day8b,
    day8b_tim,
}
