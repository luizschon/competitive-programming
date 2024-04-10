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
    let mut police = 0;
    let mut crime = 0;
    for _ in 0..n {
        let i = scan.next::<i32>();
        if i > 0 {
            police += i;
        } else if police == 0 {
            crime += 1
        } else {
            police -= 1;
        }
    }
    writeln!(out, "{}", crime);
}
