pub fn main() {
    let mut lx = include_str!("./day4.input").lines();

    let balls = lx
        .next()
        .unwrap()
        .split(',')
        .map(|b| b.parse().unwrap())
        .collect::<Vec<u8>>();

    let _ = lx.next();

    let mut cards = vec![];
    let mut card = (0u128, [0u128; 10]);
    let mut l = 0;
    let mut c = 5;

    loop {
        match lx.next() {
            None => {
                cards.push(card);
                break;
            }
            Some(line) if line.is_empty() => {
                l = 0;
                c = 5;
                cards.push(card);
                card = (0, [0; 10]);
                continue;
            }
            Some(line) => {
                for n in line.split_whitespace() {
                    let n = n.trim().parse::<u8>().unwrap();
                    card.0 |= 1 << n;
                    card.1[l] |= 1 << n;
                    card.1[c] |= 1 << n;
                    c += 1;
                }
                l += 1;
                c = 5;
            }
        }
    }

    // println!("{:?}", res);
    let mut answ: u64 = 0;

    for ball in &balls {
        for (tik, wins) in cards.iter_mut() {
            *tik &= !(1 << ball);
            for win in wins {
                if *win & *tik == 0 {
                    for i in 0..100 {
                        if (*tik >> i) & 1 == 1 {
                            answ += dbg!(i);
                        }
                    }
                    answ *= *ball as u64;

                    println!("{:}", answ);
                    return;
                }
            }
        }
    }

    // let nums = lx
    //     .next()
    //     .unwrap()
    //     .split(',')
    //     .map(|n| n.parse::<u8>().unwrap())
    //     .fold(0u128, |nums, ball| nums | 1 << ball);

    // println!("{:?}", balls);
    // dbg!(tickets);
}
