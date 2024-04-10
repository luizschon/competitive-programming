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

        let cal = (0..4).map(|_| scan.next::<usize>()).collect::<Vec<usize>>();
        let s = scan.next::<String>();
        let mut total = 0;
        for c in s.chars() {
            total += cal[c as usize - '1' as usize];
        }
        writeln!(out, "{}", total).unwrap();
}
