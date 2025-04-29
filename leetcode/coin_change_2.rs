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

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let len = coins.len();
    let mut dp = vec![0; amount + 1];
    // Caso base da recursão.
    dp[0] = 1;

    // Ordena lista de moedas
    let mut coins = coins;
    coins.sort_unstable();

    for a in 1..=amount {
        dp[a] = coins
            .iter()
            .take_while(|&c| *c as usize <= a)
            .map(|&c| dp[a - c as usize])
            .sum();
    }

    println!("{:?}", dp);

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

    run!("Bottom-up approach", change(amount, coins.clone()));
}
