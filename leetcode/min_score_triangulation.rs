// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    i32,
    io::{stdin, stdout, BufRead, BufWriter, Write},
    ops::Range,
    thread::current,
    time::Instant,
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

fn min_score_triangulation_topdown(values: Vec<i32>) -> i32 {
    type Memo = HashMap<(usize, usize), i32>;

    if values.len() < 3 {
        return 0;
    }

    fn solve(memo: &mut Memo, values: &[i32], curr_idx: usize) -> i32 {
        let len = values.len();

        if len < 3 {
            return 0;
        } else if len == 3 {
            let res = values.iter().fold(1, |acc, v| acc * v);
            return res;
        }

        if let Some(&res) = memo.get(&(curr_idx, len)) {
            return res;
        }

        let mut res = i32::MAX;
        for (i, &v) in values[1..len - 1].iter().enumerate() {
            let k = i + 1;
            res = res.min(
                solve(memo, &[values[0], v, values[len - 1]], curr_idx)
                    + solve(memo, &values[..=k], curr_idx)
                    + solve(memo, &values[k..], curr_idx + k),
            )
        }
        memo.insert((curr_idx, len), res);
        res
    }

    let mut memo = Memo::new();
    solve(&mut memo, &values, 0)
}

fn min_score_triangulation_bottomup(values: Vec<i32>) -> i32 {
    let len = values.len();
    let (rows, cols) = (len + 1, len + 1);
    let mut dp = vec![vec![0; cols]; rows];

    // Initialize all trivial triangles in dp
    for (i, w) in values.windows(3).enumerate() {
        dp[i][3] = w.iter().fold(1, |acc, v| acc * v);
    }

    // Update the dynamic programming matrix.
    for curr_len in 4..cols {
        for curr_idx in 0..cols - curr_len {
            let mut res = i32::MAX;
            for k in curr_idx + 1..curr_idx + curr_len - 1 {
                res = res.min(
                    (values[curr_idx] * values[k] * values[curr_idx + curr_len - 1])
                        + dp[curr_idx][k - curr_idx + 1]
                        + dp[k][curr_idx + curr_len - k],
                )
            }
            dp[curr_idx][curr_len] = res;
        }
    }

    *dp[0].last().unwrap()
}

fn main() {
    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}", time.elapsed());
        };
    }

    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let vertices = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();

    run!(
        "Top-down approach",
        min_score_triangulation_topdown(vertices.clone())
    );

    run!(
        "Bottom-up approach",
        min_score_triangulation_bottomup(vertices)
    );
}
