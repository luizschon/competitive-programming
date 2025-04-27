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

fn minimum_cost(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
    type Memo<'a> = HashMap<(&'a [i32], &'a [i32]), i32>;
    // (hcut, vcut) -> val
    let mut memo = Memo::new();

    fn recursion<'a>(memo: &mut Memo<'a>, hcut: &'a [i32], vcut: &'a [i32]) -> i32 {
        if hcut.len() == 0 && vcut.len() == 0 {
            return 0;
        }
        if let Some(cost) = memo.get(&(hcut, vcut)) {
            return *cost;
        }

        let mut cost = i32::MAX;
        for (i, hcost) in hcut.iter().enumerate() {
            cost = cost.min(
                hcost + recursion(memo, &hcut[..i], vcut) + recursion(memo, &hcut[i + 1..], vcut),
            )
        }
        for (j, vcost) in vcut.iter().enumerate() {
            cost = cost.min(
                vcost + recursion(memo, hcut, &vcut[..j]) + recursion(memo, hcut, &vcut[j + 1..]),
            )
        }
        memo.insert((hcut, vcut), cost);
        cost
    }

    recursion(&mut memo, &horizontal_cut, &vertical_cut)
}

fn minimum_cost_greedy(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
    let (mut hcuts, mut vcuts) = (1, 1);
    let (mut i, mut j) = (0, 0);
    let mut total_cost = 0;

    loop {
        total_cost += match (horizontal_cut.get(i), vertical_cut.get(j)) {
            (Some(hcost), Some(vcost)) => {
                if hcost > vcost {
                    hcuts += 1;
                    i += 1;
                    hcost * vcuts
                } else {
                    vcuts += 1;
                    j += 1;
                    vcost * hcuts
                }
            }
            (Some(hcost), None) => {
                i += 1;
                hcost * vcuts
            }
            (None, Some(vcost)) => {
                j += 1;
                vcost * hcuts
            }
            _ => break,
        }
    }
    total_cost
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let (m, n) = (scan.next::<i32>(), scan.next::<i32>());
    let hcut = (0..m - 1).map(|_| scan.next::<i32>()).collect::<Vec<_>>();
    let vcut = (0..n - 1).map(|_| scan.next::<i32>()).collect::<Vec<_>>();

    macro_rules! run {
        ($desc:literal, $e:expr) => {
            let time = Instant::now();
            println!("{}: {}", $desc, $e);
            println!("Elapsed time: {:?}", time.elapsed());
        };
    }

    run!(
        "Top-down memo approach",
        minimum_cost(m, n, hcut.clone(), vcut.clone())
    );
    run!("Greedy approach", minimum_cost_greedy(m, n, hcut, vcut));
}
