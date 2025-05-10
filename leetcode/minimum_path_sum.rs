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

fn min_path_sum_bf(grid: &[&[i32]]) -> i32 {
    fn solve(grid: &[&[i32]], curr_row: usize, curr_col: usize) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let n = grid[curr_row][curr_col];

        if curr_row == rows - 1 && curr_col == cols - 1 {
            return n;
        }

        let mut ans = i32::MAX;
        if curr_row + 1 < rows {
            ans = ans.min(n + solve(grid, curr_row + 1, curr_col));
        }
        if curr_col + 1 < cols {
            ans = ans.min(n + solve(grid, curr_row, curr_col + 1));
        }
        ans
    }

    solve(grid, 0, 0)
}

fn min_path_sum_td(grid: &[&[i32]]) -> i32 {
    fn solve(memo: &mut [Option<i32>], grid: &[&[i32]], curr_row: usize, curr_col: usize) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let n = grid[curr_row][curr_col];

        if curr_row == rows - 1 && curr_col == cols - 1 {
            return n;
        }

        if let Some(ans) = memo[curr_row * cols + curr_col] {
            return ans;
        }

        let mut ans = i32::MAX;
        if curr_row + 1 < rows {
            ans = ans.min(n + solve(memo, grid, curr_row + 1, curr_col));
        }
        if curr_col + 1 < cols {
            ans = ans.min(n + solve(memo, grid, curr_row, curr_col + 1));
        }
        memo[curr_row * cols + curr_col] = Some(ans);
        ans
    }

    let mut memo = vec![None; grid.len() * grid[0].len()];
    solve(&mut memo, grid, 0, 0)
}

fn min_path_sum_bu(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![i32::MAX; cols]; rows];

    // Caso base: primeiro elemento do grid.
    dp[0][0] = grid[0][0];

    for i in 0..rows {
        for j in 0..cols {
            if i + 1 < rows {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + grid[i + 1][j]);
            }
            if j + 1 < cols {
                dp[i][j + 1] = dp[i][j + 1].min(dp[i][j] + grid[i][j + 1]);
            }
        }
    }
    dp[rows - 1][cols - 1]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let matrix = stdin()
        .lines()
        .map_while(|l| {
            let s = l.unwrap().trim().to_owned();
            (s.len() != 0).then_some(s)
        })
        .map(|s| {
            s.split_whitespace()
                .map(|seg| seg.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let matrix = matrix.iter().map(|v| v.as_slice()).collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Brute-force", min_path_sum_bf(&matrix));
    run!("Top-down momoized", min_path_sum_td(&matrix));
    run!("Bottom-up", min_path_sum_bu(&matrix));
}
