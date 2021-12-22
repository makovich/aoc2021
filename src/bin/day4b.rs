pub fn main() {
    let (balls, lines) = include_str!("./day4.input").split_once("\n\n").unwrap();

    let balls = balls
        .split(',')
        .map(|b| b.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut cards = lines.split("\n\n").fold(vec![], |mut cards, line| {
        let mut card = (0u128, [0u128; 10]);

        line.split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .enumerate()
            .for_each(|(i, n)| {
                card.0 |= 1 << n;
                card.1[i / 5] |= 1 << n;
                card.1[5 + i % 5] |= 1 << n;
            });

        cards.push(card);
        cards
    });

    let mut win_answs = Vec::new();
    // let mut win_cards = HashSet::new();

    for ball in balls.iter() {
        for (card, wins) in cards.iter_mut() {
            if *card == 0 {
                continue;
            }
            // if win_cards.contains(card) {
            // continue;
            // }
            *card &= !(1 << ball);
            for win in wins {
                if *win & *card != 0 {
                    continue;
                }
                // let answ = *ball * (0..100).filter(|i| *card >> i & 1 == 1).sum::<u32>();
                win_answs.push((*card, *ball));
                // win_cards.insert(*card);
                *card = 0;
                break;
            }
        }
    }

    let (card, ball) = win_answs.last().unwrap();
    let answ = ball * (0..100).filter(|i| card >> i & 1 == 1).sum::<u32>();
    println!("{}", answ);
}
