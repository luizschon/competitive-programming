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

fn num_ways_leap_recursive(to: usize) -> u64 {
    fn solve(memo: &mut [u64], to: usize) -> u64 {
        if to == 2 {
            return 2;
        }
        if to == 1 {
            return 1;
        }
        if memo[to] != 0 {
            return memo[to];
        }
        memo[to] = solve(memo, to - 1) + solve(memo, to - 2);
        memo[to]
    }

    let mut memo = vec![0; to + 1];
    solve(&mut memo, to)
}

fn num_ways_leap(to: usize) -> u64 {
    let mut dp = vec![0; to];
    dp[0] = 1;
    dp[1] = 2;

    for i in 2..to {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[to - 1]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}", time.elapsed());
        };
    }

    run!("Answer brute-force", num_ways_leap_recursive(n));
    run!("Answer", num_ways_leap(n));
}
