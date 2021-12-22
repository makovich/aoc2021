const WIDTH: usize = 12;
const NORM: i32 = 1000 / 2;

pub fn main() {
    let res = include_str!("./day3.input")
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .fold([0; WIDTH], |mut acc, num| {
            acc.iter_mut()
                .enumerate()
                .for_each(|(i, c)| *c += (num & 1 << i) >> i);
            acc
        })
        .iter()
        .enumerate()
        .fold((0, 0), |(g, e), (idx, &cnt)| {
            let bit = cnt / NORM;
            (g + (bit << idx), e + ((bit ^ 1) << idx))
        });

    dbg!(res.0 * res.1);

    main_1st_attempt();
}

pub fn main_1st_attempt() {
    let (total, bits) = include_str!("./day3.input")
        .lines()
        .map(AsRef::<[u8]>::as_ref)
        .map(TryInto::<[u8; 12]>::try_into)
        .map(Result::unwrap)
        .map(|a| a.map(|v| v - b'0'))
        .fold(
            (0u32, [0u32; 12]),
            |(t, [aa, ab, ac, ad, ae, af, ag, ah, ai, aj, ak, al]),
             [a, b, c, d, e, f, g, h, i, j, k, l]| {
                (
                    t + 1,
                    [
                        aa + a as u32,
                        ab + b as u32,
                        ac + c as u32,
                        ad + d as u32,
                        ae + e as u32,
                        af + f as u32,
                        ag + g as u32,
                        ah + h as u32,
                        ai + i as u32,
                        aj + j as u32,
                        ak + k as u32,
                        al + l as u32,
                    ],
                )
            },
        );

    let gamma = bits.map(|b| if total / 2 < b { b'1' } else { b'0' });
    let epsilon = bits.map(|b| if total / 2 > b { b'1' } else { b'0' });

    let gamma =
        i32::from_str_radix(unsafe { std::str::from_utf8_unchecked(gamma.as_ref()) }, 2).unwrap();
    let epsilon = i32::from_str_radix(
        unsafe { std::str::from_utf8_unchecked(epsilon.as_ref()) },
        2,
    )
    .unwrap();

    dbg!(gamma);
    dbg!(epsilon);
    dbg!(gamma * epsilon);
}
