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

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn flood(grid: &Vec<Vec<char>>, visited: &mut [bool], coords: (usize, usize)) {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut stack = Vec::with_capacity(rows * cols);
        stack.push(coords);

        while let Some((i, j)) = stack.pop() {
            let vis_idx = i * cols + j;

            if visited[vis_idx] || grid[i][j] == '0' {
                continue;
            }
            visited[vis_idx] = true;

            stack.push((i - 1, j));
            stack.push((i + 1, j));
            stack.push((i, j - 1));
            stack.push((i, j + 1));
        }
    }

    let (o_rows, o_cols) = (grid.len(), grid[0].len());
    let mut padded = vec![vec!['0'; o_cols + 2]; o_rows + 2];

    for i in 1..=o_rows {
        for j in 1..=o_cols {
            padded[i][j] = grid[i - 1][j - 1];
        }
    }

    let (rows, cols) = (padded.len(), padded[0].len());
    let mut visited = vec![false; rows * cols];
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let vis_idx = i * cols + j;
            if padded[i][j] == '1' && !visited[vis_idx] {
                count += 1;
                flood(&padded, &mut visited, (i, j));
            }
        }
    }
    count
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let (m, n) = (scan.next::<i32>(), scan.next::<i32>());
    let grid = (0..m)
        .map(|_| (0..n).map(|_| scan.next::<char>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Answer", num_islands(grid));
}
