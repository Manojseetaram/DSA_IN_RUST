#![allow(dead_code)]
use std::io;
pub fn range_sum_query() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let _n: usize = a.trim().parse().unwrap();
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
        let l: i32 = b.next().unwrap().trim().parse().unwrap();
        let r: i32 = b.next().unwrap().trim().parse().unwrap();
        let mut i = l - 1;
        let mut sum = 0;
        while i <= r - 1 {
            sum += arr[i as usize];
            i += 1;
        }
        println!("{}", sum)
    }
}
pub fn even_sum_query() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let _n: usize = a.trim().parse().unwrap();
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
        let mut k = a.trim().split_whitespace();
        let l: i32 = k.next().unwrap().trim().parse().unwrap();
        let r: i32 = k.next().unwrap().trim().parse().unwrap();
        let mut i = l - 1;
        let mut sum = 0;
        while i <= r - 1 {
            if (i + 1) % 2 == 0 {
                sum += arr[i as usize] as i64;
            }
            i += 1;
        }
        println!("{}", sum)
    }
}
pub fn count_vowels() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let str_line: Vec<char> = a.trim().chars().collect();
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
pub fn prefix_sum() {
    /* range_sum_query(); */
    /*   even_sum_query(); */
    count_vowels();
}
