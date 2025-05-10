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

fn unique_paths_with_obstacles_bf(obstacle_grid: &[&[i32]]) -> i32 {
    fn solve(grid: &[&[i32]], curr_row: usize, curr_col: usize) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let pos = grid[curr_row][curr_col];

        // Chegamos no destino
        if curr_row >= rows - 1 && curr_col >= cols - 1 {
            return 1;
        }
        // Encontramos um obstaculo
        if pos == 1 {
            return 0;
        }

        let mut ans = 0;
        if curr_row + 1 < rows {
            ans += solve(grid, curr_row + 1, curr_col);
        }
        if curr_col + 1 < cols {
            ans += solve(grid, curr_row, curr_col + 1);
        }
        ans
    }

    solve(obstacle_grid, 0, 0)
}

fn unique_paths_with_obstacles_td(obstacle_grid: &[&[i32]]) -> i32 {
    type Memo = Vec<Option<i32>>;

    fn solve(memo: &mut Memo, grid: &[&[i32]], curr_row: usize, curr_col: usize) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let pos = grid[curr_row][curr_col];

        // Encontramos um obstaculo.
        if pos == 1 {
            return 0;
        }
        // Chegamos no destino.
        if curr_row >= rows - 1 && curr_col >= cols - 1 {
            return 1;
        }
        // Retorna valor memoizado, se existir.
        if let Some(ans) = memo[curr_row * cols + curr_col] {
            return ans;
        }

        let mut ans = 0;
        if curr_row + 1 < rows {
            ans += solve(memo, grid, curr_row + 1, curr_col);
        }
        if curr_col + 1 < cols {
            ans += solve(memo, grid, curr_row, curr_col + 1);
        }
        memo[curr_row * cols + curr_col] = Some(ans);
        ans
    }

    let mut memo = vec![None; obstacle_grid.len() * obstacle_grid[0].len()];
    solve(&mut memo, obstacle_grid, 0, 0)
}

fn unique_paths_with_obstacles_bu(obstacle_grid: &[&[i32]]) -> i32 {
    let (rows, cols) = (obstacle_grid.len(), obstacle_grid[0].len());

    // Matrix do mesmo tamanho que o grid, porem com padding na esquerda e em
    // cima para evitar operações de bound-checking.
    let mut dp = vec![vec![0; cols + 1]; rows + 1];
    // Caso base, topo esquerdo do grid:
    dp[0][1] = 1;

    for i in 1..=rows {
        for j in 1..=cols {
            if obstacle_grid[i - 1][j - 1] == 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }
    dp[rows][cols]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let (rows, cols) = (scan.next::<usize>(), scan.next::<usize>());

    let obstacle_grid: Vec<Vec<i32>> = (0..rows)
        .map(|_| (0..cols).map(|_| scan.next::<i32>()).collect())
        .collect();

    let obstacle_grid = obstacle_grid.iter().map(Vec::as_slice).collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!(
        "Brute-force",
        unique_paths_with_obstacles_bf(&obstacle_grid)
    );
    run!("Top-down", unique_paths_with_obstacles_td(&obstacle_grid));
    run!("Bottom-up", unique_paths_with_obstacles_bu(&obstacle_grid));
}
