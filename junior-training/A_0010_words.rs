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

    let s = scan.next::<String>();
    let mut lower = 0;
    for c in s.chars() {
        if c >= 'a' {
            lower += 1;
        }
    }
    if s.len() % 2 == 0 {
        if lower >= s.len()/2 {
            writeln!(out, "{}", s.to_ascii_lowercase());
        } else {
            writeln!(out, "{}", s.to_ascii_uppercase());
        }
    } else if lower > s.len()/2 {
        writeln!(out, "{}", s.to_ascii_lowercase());
    } else {
        writeln!(out, "{}", s.to_ascii_uppercase());
    }
}
