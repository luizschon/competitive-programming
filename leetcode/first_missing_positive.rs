// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    i32,
    io::{stdin, stdout, BufRead, BufWriter, Write},
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

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    // We can implement a counting sort which has time complexity
    // of O(n+k) and memory complexity of O(k). But, given the problem
    // constraints, k = 2^31 - 1
    let mut count = vec![0; i32::MAX as usize];
    nums.iter().for_each(|&n| {
        if n > 0 {
            count[n as usize - 1] += 1;
        }
    });

    for (i, &c) in count.iter().enumerate() {
        if c == 0 {
            return i as i32 + 1;
        }
    }
    i32::MAX
}

pub fn first_missing_positive_v2(nums: Vec<i32>) -> i32 {
    // We can implement a counting sort which has time complexity
    // of O(n+k) and memory complexity of O(k). But, given the problem
    // constraints, k = max(nums[n]) - min(nums[n]), meaning O(k) = O(max(nums))

    let max = nums
        .iter()
        .fold(0, |max, &n| if n > 0 { max.max(n) } else { max });

    let mut count = vec![0; max as usize + 1];
    nums.iter().for_each(|&n| {
        if n > 0 {
            count[n as usize - 1] += 1;
        }
    });

    for (i, &c) in count.iter().enumerate() {
        if c == 0 {
            return i as i32 + 1;
        }
    }
    max + 1
}

pub fn first_missing_positive_cycle_sort(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    let mut i = 0;
    while i < nums.len() {
        let correct_idx = nums[i] as usize - 1;

        if nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[correct_idx] {
            nums.swap(i, correct_idx);
        } else {
            i += 1
        }
    }

    for (i, &n) in nums.iter().enumerate() {
        let i = i as i32;
        if n != i + 1 {
            return i + 1;
        }
    }
    nums.len() as i32 + 1
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut s = String::new();
    let _ = stdin().lock().read_line(&mut s).unwrap();

    let nums = s
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    macro_rules! run {
        ($s:literal, $val:expr) => {
            let time = Instant::now();
            println!("{}: {}", $s, $val);
            println!("Elapsed time: {:?}\n", time.elapsed());
        };
    }

    run!("Version 1", first_missing_positive(nums.clone()));
    run!("Version 2", first_missing_positive_v2(nums.clone()));
    run!("Version 3", first_missing_positive_cycle_sort(nums));
}
