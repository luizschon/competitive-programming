// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, BufWriter, Write},
    thread::current,
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

pub fn length_of_lis_top_down(nums: Vec<i32>) -> i32 {
    type Memo = HashMap<usize, i32>;

    fn subproblem(memo: &mut Memo, nums: &[i32], prev: Option<i32>, curr_idx: usize) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if let Some(&max_len) = memo.get(&curr_idx) {
            return max_len;
        }

        let prev = prev.unwrap_or(i32::MIN);
        let mut max_len = 0;

        for (i, &n) in nums.iter().enumerate().filter(|(_, &v)| v > prev) {
            max_len = max_len.max(1 + subproblem(memo, &nums[i + 1..], Some(n), curr_idx + i + 1));
        }
        max_len
    }

    let mut memo = Memo::new();
    subproblem(&mut memo, &nums, None, 0)
}

pub fn length_of_lis_bottom_up(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];
    let mut max_len = 0;

    for i in (0..nums.len()).rev() {
        let mut sub = 0;
        for j in i + 1..nums.len() {
            if nums[j] > nums[i] {
                sub = sub.max(dp[j]);
            }
        }
        dp[i] = 1 + sub;
        max_len = max_len.max(dp[i]);
    }
    max_len
}

pub fn length_of_lis_binary_search(nums: Vec<i32>) -> i32 {
    let mut lis = Vec::<i32>::with_capacity(nums.len());

    for n in nums {
        let next_idx = lis.partition_point(|&s| s < n);
        if let Some(last) = lis.get_mut(next_idx) {
            *last = n
        } else {
            lis.push(n);
        }
    }
    lis.len() as i32
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut input = String::new();
    let _ = stdin().lock().read_line(&mut input);
    let vec = input
        .split_whitespace()
        .filter_map(|d| d.parse::<i32>().ok())
        .collect::<Vec<_>>();

    writeln!(out, "{}", length_of_lis_binary_search(vec)).unwrap();
}
