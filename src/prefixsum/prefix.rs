#![allow(dead_code)]
use std::io;

pub fn prefix_sum() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i32> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let t: usize = a.trim().parse().unwrap();
    for _ in 0..t {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut b = a.trim().split_whitespace();
        let l: i64 = b.next().unwrap().trim().parse().unwrap();
        let r: i64 = b.next().unwrap().trim().parse().unwrap();
    }
}
