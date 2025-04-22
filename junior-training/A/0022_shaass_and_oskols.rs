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
    let mut wires = (0..n).map(|_| scan.next::<i32>()).collect::<Vec<_>>();
    let m = scan.next::<i32>();
    let shots: Vec<_> = (0..m)
        .map(|_| (scan.next::<usize>() - 1, scan.next::<i32>()))
        .collect();

    for (i, j) in shots {
        if i > 0 {
            wires[i - 1] += j - 1;
        }
        if i < (n - 1) {
            wires[i + 1] += wires[i] - j;
        }
        wires[i] = 0;
    }
    wires.iter().for_each(|&n| writeln!(out, "{n}").unwrap());
}
