// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
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

    let _n = scan.next::<usize>();
    let games = scan.next::<String>();
    let mut d = 0;
    let mut a = 0;
    for g in games.chars() {
        if g == 'A' {
            a += 1;
        } else {
            d += 1;
        }
    }
    if a > d {
        writeln!(out, "Anton");
    } else if d > a {
        writeln!(out, "Danik");
    } else {
        writeln!(out, "Friendship");
    }
}
