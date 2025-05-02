// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    i32,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Activity {
    Gym,
    Contest,
}

fn solve(a: &[i32]) -> i32 {
    type Memo<'a> = HashMap<(Option<Activity>, &'a [i32]), i32>;

    fn maximum_occupied_days<'a>(
        memo: &mut Memo<'a>,
        a: &'a [i32],
        last_activity: Option<Activity>,
    ) -> i32 {
        if a.len() == 0 {
            return 0;
        }

        if let Some(&max) = memo.get(&(last_activity, a)) {
            return max;
        }

        let curr = a[0];

        let max = match last_activity {
            Some(Activity::Gym) => {
                let mut days = maximum_occupied_days(memo, &a[1..], None);
                if curr == 3 || curr == 1 {
                    days =
                        days.max(1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Contest)))
                }
                days
            }
            Some(Activity::Contest) => {
                let mut days = maximum_occupied_days(memo, &a[1..], None);
                if curr == 3 || curr == 2 {
                    days = days.max(1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Gym)));
                }
                days
            }
            None => {
                let days = maximum_occupied_days(memo, &a[1..], None);
                days.max(match curr {
                    3 => max(
                        1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Gym)),
                        1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Contest)),
                    ),
                    2 => 1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Gym)),
                    1 => 1 + maximum_occupied_days(memo, &a[1..], Some(Activity::Contest)),
                    _ => 0,
                })
            }
        };
        memo.insert((last_activity, a), max);
        max
    }

    let mut memo = Memo::new();
    maximum_occupied_days(&mut memo, a, None)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<i32>();
    let a = (0..n).map(|_| scan.next::<i32>()).collect::<Vec<_>>();

    writeln!(out, "{}", n - solve(&a)).unwrap();
}
