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

fn longest_palindrome_bf(s: &str) -> String {
    fn solve(s: &[u8], left: usize, right: usize) -> (usize, usize) {
        if left == right - 1 || left == right {
            if s[left] == s[right] {
                return (left, right);
            } else {
                return (left, left);
            }
        }

        let ans = if s[left] == s[right] {
            let mut ans = solve(s, left + 1, right - 1);
            if ans.0 == left + 1 && ans.1 == right - 1 {
                ans = (left, right)
            }
            ans
        } else {
            (0, 0)
        };

        let max = std::cmp::max_by(
            solve(s, left, right - 1),
            solve(s, left + 1, right),
            |a, b| (a.1 - a.0).cmp(&(b.1 - b.0)),
        );

        std::cmp::max_by(ans, max, |a, b| (a.1 - a.0).cmp(&(b.1 - b.0)))
    }

    let (left, right) = (0, s.len() - 1);
    let (start, end) = solve(s.as_bytes(), left, right);
    String::from(&s[start..=end])
}

fn longest_palindrome_td(s: &str) -> String {
    type Memo = std::collections::HashMap<(usize, usize), (usize, usize)>;

    fn solve(memo: &mut Memo, s: &[u8], left: usize, right: usize) -> (usize, usize) {
        if left == right - 1 || left == right {
            if s[left] == s[right] {
                return (left, right);
            } else {
                return (left, left);
            }
        }

        if let Some(ans) = memo.get(&(left, right)) {
            return *ans;
        }

        let ans = if s[left] == s[right] {
            let mut ans = solve(memo, s, left + 1, right - 1);
            if ans.0 == left + 1 && ans.1 == right - 1 {
                ans = (left, right)
            }
            ans
        } else {
            (0, 0)
        };

        let max = std::cmp::max_by(
            solve(memo, s, left, right - 1),
            solve(memo, s, left + 1, right),
            |a, b| (a.1 - a.0).cmp(&(b.1 - b.0)),
        );

        let ans = std::cmp::max_by(ans, max, |a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));
        memo.insert((left, right), ans);
        ans
    }

    let mut memo = Memo::new();
    let (left, right) = (0, s.len() - 1);
    let (start, end) = solve(&mut memo, s.as_bytes(), left, right);
    String::from(&s[start..=end])
}

fn longest_palindrome_bu(s: &str) -> String {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut dp = vec![vec![true; n]; n];

    let mut range = (0, 0);

    for end in 1..n {
        for start in 0..end {
            let curr_len = end - start + 1;
            dp[end][start] =
                (bytes[start] == bytes[end]) && (curr_len <= 2 || dp[end - 1][start + 1]);

            if dp[end][start] {
                range = std::cmp::max_by(range, (start, end), |a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));
            }
        }
    }

    let (start, end) = range;
    String::from(&s[start..=end])
}

fn longest_palindrome_alt(s: &str) -> String {
    fn expand_palindrome(s: &str, left: usize, right: usize) -> &str {
        let (mut left, mut right) = (left, right);
        let bytes = s.as_bytes();

        while (left <= right && right < bytes.len() && bytes[left] == bytes[right]) {
            left -= 1;
            right += 1;
        }
        &s[left + 1..=right - 1]
    }

    let mut ans = "";
    for i in 0..s.len() {
        let biggest = std::cmp::max_by_key(
            expand_palindrome(s, i, i),
            expand_palindrome(s, i, i + 1),
            |x| x.len(),
        );

        ans = std::cmp::max_by_key(ans, biggest, |x| x.len());
    }
    String::from(ans)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let s = scan.next::<String>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Brute-force", longest_palindrome_bf(&s));
    run!("Top-down", longest_palindrome_td(&s));
    run!("Bottom-up", longest_palindrome_bu(&s));
    run!("Two-pointer", longest_palindrome_alt(&s));
}
