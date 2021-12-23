use std::collections::HashMap;

pub fn main() {
    let graph = include_str!("./day12.input")
        .lines()
        .fold(HashMap::new(), |mut g, l| {
            let (a, b) = l.split_once('-').unwrap();
            g.entry(a).or_insert(vec![]).push(b);
            g.entry(b).or_insert(vec![]).push(a);
            g
        });

    let mut s = vec!["start"];
    let n = path(&graph, "start", &mut s, true);

    dbg!(n);
}

fn path(
    graph: &HashMap<&str, Vec<&'static str>>,
    from: &str,
    stack: &mut Vec<&str>,
    extra: bool,
) -> u32 {
    graph[from].iter().fold(0, |acc, &to| match to {
        "start" => acc,
        "end" => acc + 1,
        _ if to.starts_with(char::is_lowercase) && stack.contains(&to) => {
            if extra {
                stack.push(to);
                let p = path(graph, to, stack, false);
                stack.pop();
                acc + p
            } else {
                acc
            }
        }
        _ => {
            stack.push(to);
            let p = path(graph, to, stack, extra);
            stack.pop();
            acc + p
        }
    })
}
