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

pub fn change_td(amount: i32, coins: Vec<i32>) -> i32 {
    type Memo = HashMap<(i32, usize), i32>;

    fn solve(memo: &mut Memo, amount: i32, coins: &[i32], curr_idx: usize) -> i32 {
        let Some(curr_coin) = coins.first() else {
            return 0;
        };

        if let Some(val) = memo.get(&(amount, curr_idx)) {
            return *val;
        }

        if amount - curr_coin == 0 {
            return 1;
        } else if amount - curr_coin < 0 {
            return 0;
        }

        // We can take the coin or skip it and never use it again
        let ans = solve(memo, amount - curr_coin, coins, curr_idx)
            + solve(memo, amount, &coins[1..], curr_idx + 1);
        memo.insert((amount, curr_idx), ans);
        ans
    }

    let mut coins = coins;
    coins.sort_unstable();
    let mut memo = Memo::new();
    solve(&mut memo, amount, &coins, 0)
}

pub fn change_bu(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![0; amount + 1];

    // Base case: there are 1 way to make 0 out of any list o coins
    dp[0] = 1;

    // Does any ordering work for this problem?
    //
    // - Yes, as long as we update the states in dp
    // for each coin only once.

    for coin in coins {
        for val in coin as usize..=amount {
            dp[val] += dp[val - coin as usize];
        }
    }
    dp[amount]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let amount = scan.next::<i32>();
    let mut buf = String::new();
    stdin().read_line(&mut buf);
    let coins = buf
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}", time.elapsed());
        };
    }

    run!("Top-down approach", change_td(amount, coins.clone()));
    run!("Bottom-up approach", change_bu(amount, coins.clone()));
}
