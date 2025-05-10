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

fn length_of_lis_bf(nums: &[i32]) -> i32 {
    fn solve(nums: &[i32], prev: Option<i32>) -> i32 {
        let Some(curr) = nums.first().copied() else {
            return 0;
        };

        if curr > prev.unwrap_or(i32::MIN) {
            1 + solve(&nums[1..], Some(curr))
        } else {
            max(solve(&nums[1..], prev), solve(&nums[1..], Some(curr)))
        }
    }

    solve(nums, None)
}

fn length_of_lis_td(nums: &[i32]) -> i32 {
    type Memo = HashMap<(usize, Option<i32>), i32>;

    fn solve(memo: &mut Memo, nums: &[i32], prev: Option<i32>, curr_idx: usize) -> i32 {
        let Some(curr) = nums.get(curr_idx).copied() else {
            return 0;
        };

        if let Some(res) = memo.get(&(curr_idx, prev)).copied() {
            return res;
        }

        let res = if curr > prev.unwrap_or(i32::MIN) {
            1 + solve(memo, nums, Some(curr), curr_idx + 1)
        } else {
            max(
                solve(memo, nums, prev, curr_idx + 1),
                solve(memo, &nums[1..], Some(curr), curr_idx + 1),
            )
        };
        memo.insert((curr_idx, prev), res);
        res
    }

    let mut memo = Memo::new();
    solve(&mut memo, nums, None, 0)
}

fn length_of_lis_bu(nums: &[i32]) -> i32 {
    let mut dp = vec![1; nums.len()];

    let mut ans = 1;
    for (i, &curr) in nums.iter().enumerate() {
        for j in i + 1..nums.len() {
            if nums[j] > curr {
                dp[j] = dp[j].max(dp[i] + 1);
                ans = ans.max(dp[j]);
            }
        }
    }
    ans
}

fn length_of_lis_bs(nums: &[i32]) -> i32 {
    let mut lis = Vec::<i32>::with_capacity(nums.len());

    for &n in nums {
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

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let nums = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    run!("Brute-force", length_of_lis_bf(&nums));
    run!("Top-down momoized", length_of_lis_td(&nums));
    run!("Bottom-up", length_of_lis_bu(&nums));
    run!("Binary search", length_of_lis_bs(&nums));
}
