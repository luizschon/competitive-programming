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

    let y = scan.next::<i32>();
    let w = scan.next::<i32>();
    let max = y.max(w);
    let mut num = 6 - max;

    if y == w && y == max {
        num += 1;
    } else {
        if y == max {
            num += 1;
        }
        if w == max {
            num += 1;
        }
    }

    let mut dem = 6;

    if num % 2 == 0 {
        num /= 2;
        dem /= 2;
    }
    if num % 3 == 0 {
        num /= 3;
        dem /= 3;
    }

    writeln!(out, "{num}/{dem}");
}
