#![allow(dead_code)]
use std::io;

pub fn print_all_subarray() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();

    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut l = 0;
    while l < n {
        let mut r = l;
        while r < n {
            let mut i = l;
            while i <= r {
                print!("{} ", arr[i]);
                i += 1;
            }
            r += 1;
            println!()
        }
        l += 1;
    }
}
pub fn subarray() {
    print_all_subarray();
}
