// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

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

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let teams: Vec<_> = (0..n)
        .map(|_| (scan.next::<i32>(), scan.next::<i32>()))
        .collect();
    let mut ans = 0;

    for (i, team) in teams.clone().into_iter().enumerate() {
        ans += teams
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, t)| if t.0 == team.1 { 1 } else { 0 })
            .sum::<i32>();
    }
    writeln!(out, "{ans}");
}
