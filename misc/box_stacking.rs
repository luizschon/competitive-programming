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

type BoxDims = (u32, u32, u32); //(Length, Width, Height)

fn max_stack_height_bu(boxes: &[BoxDims]) -> u32 {
    let mut binding = boxes.to_vec();
    let boxes: &mut Vec<BoxDims> = binding.as_mut();
    boxes.sort_by(|a, b| a.0.cmp(&b.0).reverse().then(a.1.cmp(&b.1).reverse()));

    let mut dp = boxes.iter().map(|&(_, _, h)| h).collect::<Vec<_>>();
    let mut res = u32::MIN;

    for i in 1..boxes.len() {
        let (l1, w1, h1) = boxes[i];
        dp[i] = *dp[..i]
            .iter()
            .enumerate()
            .filter(|&(j, _)| {
                let (l, w, _) = boxes[j];
                l > l1 && w > w1
            })
            .map(|(_, h)| h)
            .max()
            .unwrap_or(&0);
        res = res.max(dp[i]);
    }
    res
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let boxes = (0..n)
        .map(|_| (scan.next::<u32>(), scan.next::<u32>(), scan.next::<u32>()))
        .collect::<Vec<BoxDims>>();

    macro_rules! run {
        ($str:literal, $e:expr) => {
            let time = std::time::Instant::now();
            println!("{}: {}", $str, $e);
            println!("Time elapsed: {:?}", time.elapsed());
        };
    }

    run!("Bottom-up approach", max_stack_height_bu(&boxes));
}
