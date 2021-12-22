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

    let mut answ = 0;

    'a: for ball in balls.iter() {
        for (card, wins) in cards.iter_mut() {
            *card &= !(1 << ball);
            for win in wins {
                if *win & *card != 0 {
                    continue;
                }
                answ = (0..100).filter(|i| *card >> i & 1 == 1).sum();
                answ *= *ball;
                break 'a;
            }
        }
    }

    println!("{:?}", answ);
}
