// vim: noai:ts=4:sw=4
use std::collections::hash_map::Entry;
use std::collections::HashMap;
#[allow(unused_imports)]
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

    let shoes = (0..4).map(|_| scan.next::<i32>()).collect::<Vec<_>>();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;

    for s in shoes {
        match map.entry(s) {
            Entry::Occupied(_) => {
                ans += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        };
    }
    writeln!(out, "{ans}");
}
