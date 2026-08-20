#![allow(dead_code)]

use std::io;
pub fn result_day() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let _n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let marks: i64 = a.trim().parse().unwrap();
    let mut i = 0;
    let mut pass = 0;
    let mut fail = 0;
    while i < arr.len() {
        if arr[i] <= marks {
            pass += 1;
        } else {
            fail += 1
        }
        i += 1;
    }
    println!("Pass: {pass}");
    println!("Fail: {fail}");
}
pub fn contest() {
    result_day();
}
