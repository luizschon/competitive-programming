// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
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

fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }

    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; cols + 1]; rows + 1];
    let mut max_size = 0;

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            dp[i][j] = (matrix[i][j] == '1') as i32
                * (1 + min(min(dp[i + 1][j], dp[i][j + 1]), dp[i + 1][j + 1]));
            max_size = max_size.max(dp[i][j]);
        }
    }
    max_size * max_size
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
}
