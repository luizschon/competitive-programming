// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    i32,
    io::{stdin, stdout, BufWriter, Read, Write},
};

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

fn min_travel_cost(table: &Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (table.len(), table[0].len());
    let mut dp = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut min_prev = None;
            if i > 0 {
                min_prev = Some(dp[i - 1][j]);
            }
            if j > 0 {
                min_prev = Some(min_prev.unwrap_or(i32::MAX).min(dp[i][j - 1]));
            }
            dp[i][j] += min_prev.unwrap_or(0) + table[i][j]
        }
    }
    dp[rows - 1][cols - 1]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let table = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Answer: {}", min_travel_cost(&table));
}
