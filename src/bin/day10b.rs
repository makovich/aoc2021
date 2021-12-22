use std::collections::HashMap;

pub fn main() {
    let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let put = |stack: &mut Vec<_>, symbl| {
        stack.push(braces[&symbl]);
        Some(Ok(String::from_iter(stack.iter())))
    };

    let mut res = include_str!("./day10.input")
        .lines()
        .filter_map(|line| {
            line.chars()
                .scan((vec![], false), |(stack, err), sym| match sym {
                    _ if *err => None,
                    '(' | '[' | '{' | '<' => put(stack, sym),
                    _ => stack.pop().map(|c| {
                        if c != sym {
                            Err(*err = true)
                        } else {
                            Ok(String::from_iter(stack.iter()))
                        }
                    }),
                })
                .last()
                .transpose()
                .ok()
                .map(Option::unwrap)
        })
        .map(|stack| stack.chars().rev().fold(0, |acc, c| acc * 5 + scores[&c]))
        .collect::<Vec<u64>>();

    res.sort();
    println!("{:?}", res);
    println!("{:?}", res[res.len() / 2]);
}
