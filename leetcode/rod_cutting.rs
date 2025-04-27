// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    i32,
    io::{stdin, stdout, BufWriter, Write},
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

fn cut_rod_top_down_hm(p: &Vec<i32>, n: usize) -> i32 {
    let mut memo = HashMap::<usize, i32>::new();

    fn max_revenue(memo: &mut HashMap<usize, i32>, p: &Vec<i32>, n: usize) -> i32 {
        if n == 0 {
            return 0;
        }

        let mut q = i32::MIN;
        for i in 0..n {
            let cut_len = n - (i + 1);

            let cost = if let Some(val) = memo.get(&cut_len) {
                *val
            } else {
                let val = max_revenue(memo, p, cut_len);
                memo.insert(cut_len, val);
                val
            };
            q = q.max(p[i] + cost);
        }
        q
    }
    max_revenue(&mut memo, p, n)
}

fn cut_rod_top_down_arr(p: &[i32], n: usize) -> i32 {
    let mut memo = vec![-1; n + 1];

    fn aux(memo: &mut [i32], p: &[i32], n: usize) -> i32 {
        if memo[n] >= 0 {
            return memo[n];
        }

        if n == 0 {
            memo[n] = 0;
            return 0;
        }

        let mut q = i32::MIN;
        for i in 0..n {
            q = q.max(p[i] + aux(memo, p, n - (i + 1)));
        }
        memo[n] = q;
        memo[n]
    }
    aux(&mut memo, p, n)
}

fn cut_rod_bottom_up(p: &[i32], n: usize) -> i32 {
    let mut memo = vec![0; n + 1];

    for i in 1..=n {
        let mut q = i32::MIN;
        for j in 0..i {
            q = q.max(p[j] + memo[i - (j + 1)]);
        }
        memo[i] = q;
    }
    memo[n]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let p = (0..n).map(|i| i as i32 + 2).collect::<Vec<_>>();

    macro_rules! run {
        ($s:literal, $fn:expr) => {
            let time = Instant::now();
            println!("{}: {}", $s, $fn);
            println!("Elapsed time: {:?}", time.elapsed());
        };
    }

    println!("Cutting-rod...");
    run!("Top-down hashmap memoized impl", cut_rod_top_down_hm(&p, n));
    run!("Top-down array memoized impl", cut_rod_top_down_arr(&p, n));
    run!("Bottom-up array impl", cut_rod_bottom_up(&p, n));
}
