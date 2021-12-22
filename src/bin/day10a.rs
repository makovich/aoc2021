use std::collections::HashMap;

pub fn main() {
    let braces = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let put = |stack: &mut Vec<_>, symbl| {
        stack.push(braces[&symbl]);
        Some(0)
    };

    let res = include_str!("./day10.input")
        .lines()
        .map(|line| {
            line.chars()
                .scan((vec![], false), |(stack, err), sym| match sym {
                    _ if *err => None,
                    '(' | '[' | '{' | '<' => put(stack, sym),
                    _ => stack.pop().map(|c| {
                        if c != sym {
                            *err = true;
                            scores[&sym]
                        } else {
                            0
                        }
                    }),
                })
                .last()
                .unwrap()
        })
        .sum::<i32>();

    println!("{}", res);
}
