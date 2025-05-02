// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StringOrd {
    Normal,
    Reversed,
}

fn solve(cost: &[(usize, i64, &str)]) -> i64 {
    type Memo = HashMap<(usize, Option<StringOrd>), Option<i64>>;

    fn min_cost(
        memo: &mut Memo,
        cost: &[(usize, i64, &str)],
        prev: Option<&str>,
        prev_ord: Option<StringOrd>,
    ) -> Option<i64> {
        let [(idx, c, s), rest @ ..] = cost else {
            return Some(0);
        };
        if let Some(min) = memo.get(&(*idx, prev_ord)) {
            return *min;
        }

        let rev_s = s.chars().rev().collect::<String>();

        let val = if let Some(prev) = prev {
            let o = &[
                match s.cmp(&prev) {
                    Ordering::Greater | Ordering::Equal => {
                        min_cost(memo, rest, Some(s), Some(StringOrd::Normal))
                    }
                    _ => None,
                },
                match rev_s.as_str().cmp(&prev) {
                    Ordering::Greater | Ordering::Equal => {
                        min_cost(memo, rest, Some(&rev_s), Some(StringOrd::Reversed))
                            .and_then(|temp| Some(c + temp))
                    }
                    _ => None,
                },
            ];
            o.iter().fold(None, |acc, v| {
                if let Some(val) = v {
                    acc.and_then(|a| Some(std::cmp::min::<i64>(a, *val)))
                        .or(Some(*val))
                } else {
                    acc
                }
            })
        } else {
            let o = &[
                min_cost(memo, rest, Some(s), Some(StringOrd::Normal)),
                min_cost(memo, rest, Some(&rev_s), Some(StringOrd::Reversed))
                    .and_then(|temp| Some(c + temp)),
            ];
            o.iter().fold(None, |acc, v| {
                if let Some(val) = v {
                    acc.and_then(|a| Some(std::cmp::min::<i64>(a, *val)))
                        .or(Some(*val))
                } else {
                    acc
                }
            })
        };

        memo.insert((*idx, prev_ord), val);
        val
    }

    let mut memo = Memo::new();
    if let Some(cost) = min_cost(&mut memo, cost, None, None) {
        cost
    } else {
        -1
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<i64>();
    let c = (0..n).map(|_| scan.next::<i64>()).collect::<Vec<_>>();
    let ss = (0..n).map(|_| scan.next::<String>()).collect::<Vec<_>>();
    let ss = ss.iter().map(|s| &**s).collect::<Vec<&str>>();

    let cost = c
        .into_iter()
        .enumerate()
        .zip(ss.into_iter())
        .map(|((i, c), s)| (i, c, s))
        .collect::<Vec<_>>();

    writeln!(out, "{}", solve(&cost)).unwrap();
}
