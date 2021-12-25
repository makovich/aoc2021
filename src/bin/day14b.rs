use std::collections::HashMap;

// tail -n+3 day14.input | cut -b1,2 | sort | uniq | wc -l
const PAIRS: usize = 100;
// tail -n+3 src/bin/day14.input | cut -d' ' -f3 | sort | uniq | wc -l
const ALPHA: usize = 10;

pub fn main() {
    let (poly, seqs) = include_str!("./day14.input").split_once("\n\n").unwrap();

    let mut res = poly
        .as_bytes()
        .iter()
        .fold(HashMap::with_capacity(ALPHA), |mut map, a| {
            *map.entry(*a).or_insert(0) += 1;
            map
        });

    eprintln!("res = {:?}", res);

    let mut poly =
        poly.as_bytes()
            .windows(2)
            .fold(HashMap::with_capacity(PAIRS), |mut map, val| {
                *map.entry((val[0], val[1])).or_insert(0) += 1u64;
                map
            });

    eprintln!("poly = {:?}", poly);

    let seqs = seqs
        .lines()
        .map(|l| {
            let (ab, c) = l.split_once(" -> ").unwrap();
            let a = ab.as_bytes()[0];
            let b = ab.as_bytes()[1];
            let c = c.as_bytes()[0];
            (a, b, c)
        })
        .fold(HashMap::with_capacity(PAIRS), |mut map, (a, b, c)| {
            map.insert((a, b), c);
            map
        });

    eprintln!("seqs = {:?}", seqs);

    let cur = &mut HashMap::with_capacity(PAIRS);
    let nxt = &mut poly;

    for _ in 0..40 {
        std::mem::swap(cur, nxt);
        nxt.clear();

        // eprintln!("cur = {:?}", cur);
        // eprintln!("nxt = {:?}", nxt);

        for ((a, b), times) in cur.iter() {
            let c = seqs[&(*a, *b)];

            *nxt.entry((*a, c)).or_insert(0) += times;
            *nxt.entry((c, *b)).or_insert(0) += times;

            *res.entry(c).or_insert(0) += times;
        }
    }

    // eprintln!("\nres={:?}", res);

    // eprintln!("min = {:?}", res.values().min());
    // eprintln!("max = {:?}", res.values().max());

    println!(
        "{:?}",
        res.values().max().unwrap() - res.values().min().unwrap()
    );
}
