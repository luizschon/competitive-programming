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

fn solve(grid: &mut Vec<Vec<char>>) {
    enum LastDirection {
        Up,
        Down,
        Left,
        Right,
    }

    fn is_in_border((i, j): (usize, usize), (rows, cols): (usize, usize)) -> bool {
        i == 0 || j == 0 || i == rows - 1 || j == cols - 1
    }

    fn surround(
        grid: &mut Vec<Vec<char>>,
        memo: &mut Vec<Option<bool>>,
        last_dir: Option<LastDirection>,
        (i, j): (usize, usize),
    ) -> bool {
        let (rows, cols) = (grid.len(), grid[0].len());
        let idx = i * cols + j;

        // Bound-checks do grid.
        if i >= rows || j >= cols {
            return false;
        }
        if let Some(is_surrounded) = memo[idx] {
            return is_surrounded;
        }
        if grid[i][j] != 'O' {
            return true;
        }
        if is_in_border((i, j), (rows, cols)) {
            return false;
        }

        use LastDirection::*;
        #[rustfmt::skip]
        let neighbors = match last_dir {
            Some(Up)    => vec![((i + 1, j), Up), ((i, j + 1), Left), ((i, j - 1), Right)],
            Some(Down)  => vec![((i - 1, j), Down), ((i, j + 1), Left), ((i, j - 1), Right)],
            Some(Left)  => vec![((i + 1, j), Up), ((i - 1, j), Down), ((i, j + 1), Left)],
            Some(Right) => vec![((i + 1, j), Up), ((i - 1, j), Down), ((i, j - 1), Right)],
            _           => vec![((i + 1, j), Up), ((i - 1, j), Down), ((i, j + 1), Left), ((i, j - 1), Right)],
        };

        let is_surrounded = neighbors
            .into_iter()
            .map(|(coords, dir)| surround(grid, memo, Some(dir), coords))
            .fold(true, |acc, x| acc && x);

        if is_surrounded {
            grid[i][j] = 'X'
        }
        memo[idx] = Some(is_surrounded);
        is_surrounded
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut memo = vec![None; rows * cols];

    // Podemos ignorar as bordas, já que um 'O' nelas nunca estará rodeados de 'X's.
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let idx = i * cols + j;
            if grid[i][j] == 'O' && memo[idx].is_none() {
                surround(grid, &mut memo, None, (i, j));
            }
        }
    }
    for k in 0..rows {
        let idx = k * cols;
        println!("{:?}", &memo[idx..idx + cols])
    }
    println!();
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let (m, n) = (scan.next::<i32>(), scan.next::<i32>());
    let mut grid = (0..m)
        .map(|_| (0..n).map(|_| scan.next::<char>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {:?}", $str, $e);
            println!("Time elapsed: {:?}\n", time.elapsed());
        };
    }

    run!("Answer", solve(&mut grid));
    for line in grid {
        println!("{line:?}");
    }
}
