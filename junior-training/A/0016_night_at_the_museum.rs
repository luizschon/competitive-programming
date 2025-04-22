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

    let word = scan.next::<String>();
    let mut ans = 0;
    let mut curr_pos = 'a' as i32;
    for b in word.bytes() {
        ans += std::cmp::min(
            (b as i32 - curr_pos).rem_euclid(26),
            (curr_pos - b as i32).rem_euclid(26),
        );
        curr_pos = b as i32;
    }
    writeln!(out, "{ans}");
}
