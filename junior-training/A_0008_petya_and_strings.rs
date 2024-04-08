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

    let str1 = scan.next::<String>().to_ascii_uppercase();
    let str2 = scan.next::<String>().to_ascii_uppercase();
    for (i, c) in str1.chars().enumerate() {
        let s2_bytes = str2.as_bytes();
        if c < s2_bytes[i] as char {
            writeln!(out, "-1");
            return;
        } else if c > s2_bytes[i] as char {
            writeln!(out, "1");
            return;
        }
    }
    writeln!(out, "0");
}
