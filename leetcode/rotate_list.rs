// vim: noai:ts=4:sw=4
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::VecDeque,
    fmt::Display,
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![self.val];
        let mut curr = self;
        while let Some(node) = curr.next.as_ref() {
            v.push(node.val);
            curr = node;
        }
        f.debug_list().entries(v).finish()
    }
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut old_head = head.as_ref().unwrap();
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let k = scan.next::<i32>();

    let mut s = String::new();
    let _ = stdin().read_line(&mut s).unwrap();
    let nodes = s
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let mut head = None;
    let mut curr = &mut head;
    for node in nodes {
        *curr = Some(Box::new(ListNode::new(node)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    println!("{}", rotate_right(head, k).unwrap());
}
