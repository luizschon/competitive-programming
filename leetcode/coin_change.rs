// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    i32,
    io::{stdin, stdout, BufRead, BufWriter, Write},
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

fn coin_change_bf(coins: &[i32], amount: i32) -> i32 {
    fn solve(coins: &[i32], amount: i32) -> Option<i32> {
        let Some(coin) = coins.first().copied() else {
            return None;
        };

        if amount == 0 {
            return Some(0);
        } else if amount < coin {
            return None;
        }

        // Take / Skip coin
        let take = solve(coins, amount - coin).map(|c| c + 1);
        let skip = solve(&coins[1..], amount);

        match (take, skip) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), _) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        }
    }

    let mut coins = coins.to_vec();
    coins.sort_unstable();
    match solve(&coins, amount) {
        Some(v) => v,
        _ => -1,
    }
}

fn coin_change_td(coins: &[i32], amount: i32) -> i32 {
    type Memo = HashMap<(i32, usize), Option<i32>>;

    fn solve(memo: &mut Memo, coins: &[i32], amount: i32, curr_idx: usize) -> Option<i32> {
        let Some(coin) = coins.get(curr_idx).copied() else {
            return None;
        };

        if amount == 0 {
            return Some(0);
        } else if amount < coin {
            return None;
        }

        if let Some(ans) = memo.get(&(amount, curr_idx)) {
            return *ans;
        }

        // Take / Skip coin
        let take = solve(memo, coins, amount - coin, curr_idx).map(|c| c + 1);
        let skip = solve(memo, coins, amount, curr_idx + 1);

        let ans = match (take, skip) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), _) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        };
        memo.insert((amount, curr_idx), ans);
        ans
    }

    let mut memo = Memo::new();
    let mut coins = coins.to_vec();
    coins.sort_unstable();
    match solve(&mut memo, &coins, amount, 0) {
        Some(v) => v,
        _ => -1,
    }
}

fn coin_change_bu(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp: Vec<Option<i32>> = vec![None; amount + 1];
    // Caso base: amount = 0
    dp[0] = Some(0);

    for &coin in coins {
        for a in coin as usize..=amount {
            if let Some(v) = dp[a - coin as usize] {
                dp[a] = Some(dp[a].unwrap_or(i32::MAX).min(v + 1));
            }
        }
    }

    match dp[amount] {
        Some(a) => a,
        _ => -1,
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let amount = scan.next::<i32>();

    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();

    let coins = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Brute-force", coin_change_bf(&coins, amount));
    run!("Top-down momoized", coin_change_td(&coins, amount));
    run!("Bottom-up", coin_change_bu(&coins, amount));
}
