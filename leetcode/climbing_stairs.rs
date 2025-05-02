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

fn climb_stairs(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n];
    // Base cases:
    dp[0] = 1;
    if n > 1 {
        dp[1] = 2;
    }

    for i in 2..n {
        dp[i] = dp[i - 2] + dp[i - 1];
    }
    dp[n - 1]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let steps = scan.next::<i32>();
    println!("{}", climb_stairs(steps));
}
