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

    let n = scan.next::<usize>();
    let v = (0..n).map(|_| scan.next::<usize>()).collect::<Vec<usize>>();
    let mut s = 0;
    let mut d = 0;
    let mut l = 0;
    let mut r = n - 1;
    for i in 0..n {
        if i % 2 == 0 {
            if v[l] >= v[r] {
                s += v[l];
                l += 1;
            } else {
                s += v[r];
                r -= 1;
            }
        } else {
            if v[l] >= v[r] {
                d += v[l];
                l += 1;
            } else {
                d += v[r];
                r -= 1;
            }
        }
    }
    writeln!(out, "{} {}", s, d);
}
