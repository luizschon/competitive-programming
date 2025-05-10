// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    i32,
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

fn minimum_total_bf(triangle: Vec<Vec<i32>>) -> i32 {
    fn solve(triangle: &[&[i32]], curr_idx: usize) -> i32 {
        let Some(line) = triangle.first() else {
            return 0;
        };

        min(
            line[curr_idx] + solve(&triangle[1..], curr_idx),
            line[curr_idx] + solve(&triangle[1..], curr_idx + 1),
        )
    }

    let triangle = triangle.iter().map(Vec::as_slice).collect::<Vec<_>>();
    solve(&triangle, 0)
}

fn minimum_total_td(triangle: Vec<Vec<i32>>) -> i32 {
    type Memo = HashMap<(usize, usize), i32>; // (curr_line, curr_idx)

    fn solve(memo: &mut Memo, triangle: &[&[i32]], curr_line: usize, curr_idx: usize) -> i32 {
        let Some(line) = triangle.get(curr_line) else {
            return 0;
        };

        if let Some(ans) = memo.get(&(curr_line, curr_idx)).copied() {
            return ans;
        }

        let ans = min(
            line[curr_idx] + solve(memo, triangle, curr_line + 1, curr_idx),
            line[curr_idx] + solve(memo, triangle, curr_line + 1, curr_idx + 1),
        );
        memo.insert((curr_line, curr_idx), ans);
        ans
    }

    let mut memo = Memo::new();
    let triangle = triangle.iter().map(Vec::as_slice).collect::<Vec<_>>();
    solve(&mut memo, &triangle, 0, 0)
}

fn minimum_total_bu(triangle: Vec<Vec<i32>>) -> i32 {
    let lines = triangle.len();
    let mut dp = vec![vec![None; lines + 1]; lines + 1];

    let mut ans = i32::MAX;
    for i in 0..lines {
        let line = &triangle[i];

        for (j, &n) in line.iter().enumerate() {
            let (left, right) = (dp[i + 1][j], dp[i + 1][j + 1]);
            dp[i + 1][j] = Some(left.unwrap_or(i32::MAX).min(dp[i][j].unwrap_or(0) + n));
            dp[i + 1][j + 1] = Some(right.unwrap_or(i32::MAX).min(dp[i][j].unwrap_or(0) + n));
        }
    }

    dp[lines].iter().filter_map(|&x| x).min().unwrap_or(0)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let nums = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut n = 1;
    let mut iter = nums.iter();
    let mut triangle = vec![];
    'outer: loop {
        let mut temp = vec![];
        for i in 0..n {
            if let Some(num) = iter.next() {
                temp.push(*num);
            } else {
                break 'outer;
            }
        }
        triangle.push(temp);
        n += 1;
    }

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Brute-force", minimum_total_bf(triangle.clone()));
    run!("Top-down momoized", minimum_total_td(triangle.clone()));
    run!("Bottom-up", minimum_total_bu(triangle));
}
