// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    i32,
    io::{stdin, stdout, BufRead, BufWriter, Read, Write},
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

fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    let mut max = i32::MIN;

    for i in 0..nums.len() {
        dp[i] += nums[i];
        for j in i + 2..nums.len() {
            dp[j] = dp[j].max(dp[i]);
        }
        max = max.max(dp[i])
    }
    max
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();

    let houses = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", rob(houses));
}
