#![allow(dead_code)]
use std::io;

pub fn input_output() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut a = vec![0; n];

    let mut i = 0;
    while i < n {
        a[i] = nums[i];
        i += 1;
    }

    let mut i = 0;
    while i < n {
        print!("{} ", a[i]);
        i += 1;
    }
}
pub fn reverse_array() {
    let a = vec![11, 24, 87, 9];
    let mut b = Vec::new();

    let mut i = a.len();
    while i > 0 {
        b.push(a[i - 1]);
        i -= 1;
    }

    let mut j = 0;
    while j < b.len() {
        print!("{} ", b[j]);
        j += 1;
    }
    println!()
}
pub fn arrays() {
    /*   input_output(); */
    reverse_array();
}
