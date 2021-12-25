use std::collections::HashMap;

pub fn main() {
    let (poly, rules) = include_str!("./day14.input")
        .split_once("\n\n")
        .map(|(p, r)| {
            (
                p.as_bytes(),
                r.lines().fold(HashMap::<[u8; 2], u8>::new(), |mut r, l| {
                    let (k, v) = l
                        .split_once(" -> ")
                        .map(|(k, v)| ([k.as_bytes()[0], k.as_bytes()[1]], v.as_bytes()[0]))
                        .unwrap();

                    r.insert(k, v);
                    r
                }),
            )
        })
        .unwrap();

    println!("{:?}", (poly, &rules));

    let mut map = HashMap::<u8, i32>::new();
    poly.iter().for_each(|b| *map.entry(*b).or_insert(0) += 1);

    println!("{:?}", map);

    let res = (0..40).fold(poly.to_vec(), |acc, iter| {
        println!("iter={}", iter);
        acc.windows(2).fold(vec![acc[0]], |mut res, pair| {
            let mid = &rules.get(pair).unwrap();
            *map.entry(**mid).or_insert(0) += 1;
            res.push(**mid);
            res.push(pair[1]);
            res
        })
    });

    println!("{:?}", String::from_utf8(res).unwrap().len());
    println!("{:?}", map);

    let answer = map.values().max().unwrap() - map.values().min().unwrap();

    println!("{:?}", answer);
}
