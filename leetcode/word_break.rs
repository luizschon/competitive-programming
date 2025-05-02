// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashSet,
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

fn word_break_bf(s: &str, word_dict: &[&str]) -> bool {
    fn solve(s: &str, word_dict: &HashSet<&str>) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut ans = false;
        for i in 1..=s.len() {
            ans |= word_dict.contains(&s[..i]) && solve(&s[i..], word_dict);
            if ans {
                break;
            }
        }
        ans
    }

    let word_dict = HashSet::from_iter(word_dict.into_iter().map(|s| *s));
    solve(s, &word_dict)
}

fn word_break_td(s: &str, word_dict: &[&str]) -> bool {
    fn solve(
        memo: &mut [Option<bool>],
        s: &str,
        word_dict: &HashSet<&str>,
        curr_idx: usize,
    ) -> bool {
        if s.len() == 0 {
            return true;
        }

        if let Some(ans) = memo[curr_idx] {
            return ans;
        }

        let mut ans = false;
        for i in 1..=s.len() {
            ans |= word_dict.contains(&s[..i]) && solve(memo, &s[i..], word_dict, curr_idx + i);
            if ans {
                break;
            }
        }
        memo[curr_idx] = Some(ans);
        ans
    }

    let word_dict = HashSet::from_iter(word_dict.into_iter().map(|s| *s));
    let mut memo = vec![None; s.len()];
    solve(&mut memo, s, &word_dict, 0)
}

fn word_break_bu(s: &str, word_dict: &[&str]) -> bool {
    let word_dict = HashSet::<&str>::from_iter(word_dict.into_iter().map(|s| *s));
    let mut dp = vec![false; s.len() + 1];
    // Base case: empty string
    dp[0] = true;

    for i in 1..=s.len() {
        // We split the string in the i'th index and check if we can form a word break
        // with the string to the left of the i'th index (this is our subproblem, whose
        // solution we will store in the dp array).
        for j in 0..i {
            dp[i] |= dp[j] && word_dict.contains(&s[j..i]);
            if dp[i] == true {
                break;
            }
        }
    }
    dp[s.len()]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();

    let s = input.trim_end().to_owned();

    input.clear();
    stdin().lock().read_line(&mut input).unwrap();
    let word_dict = input.split_whitespace().collect::<Vec<&str>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Brute-force", word_break_bf(&s, &word_dict));
    run!("Top-down memoized", word_break_td(&s, &word_dict));
    run!("Bottom-up", word_break_td(&s, &word_dict));
}
