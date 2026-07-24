#![allow(dead_code)]
use std::io;

pub fn range_sum_query() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let arr: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut p = vec![0; n];
    p[0] = arr[0];
    for i in 1..n {
        p[i] = p[i - 1] + arr[i];
    }

    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let q: usize = s.trim().parse().unwrap();

    for _ in 0..q {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let mut it = s.split_whitespace();

        let l: usize = it.next().unwrap().parse().unwrap();
        let r: usize = it.next().unwrap().parse().unwrap();

        let ans = if l == 1 {
            p[r - 1]
        } else {
            p[r - 1] - p[l - 2]
        };

        println!("{ans}");
    }
}
pub fn even_sum_query() {
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
    let mut p = vec![0; n];
    p[0] = 0;
    for i in 1..n {
        if (i + 1) % 2 == 0 {
            p[i] = p[i - 1] + arr[i]
        } else {
            p[i] = p[i - 1];
        }
    }
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let t: usize = a.trim().parse().unwrap();
    for _ in 0..t {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut k = a.trim().split_whitespace();
        let l: usize = k.next().unwrap().trim().parse().unwrap();
        let r: usize = k.next().unwrap().trim().parse().unwrap();

        let range_sum = if l == 1 {
            p[r - 1]
        } else {
            p[r - 1] - p[l - 2]
        };
        println!("{range_sum}")
    }
}
pub fn count_vowels() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let str_line: Vec<char> = a.trim().chars().collect();
    let mut p = vec![0; n];

    if matches!(str_line[0], 'a' | 'e' | 'i' | 'o' | 'u') {
        p[0] = 1;
    }

    for i in 1..n {
        if matches!(str_line[i], 'a' | 'e' | 'i' | 'o' | 'u') {
            p[i] = p[i - 1] + 1;
        } else {
            p[i] = p[i - 1];
        }
    }
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let t: usize = a.trim().parse().unwrap();
    for _ in 0..t {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut b = a.trim().split_whitespace();
        let l: usize = b.next().unwrap().trim().parse().unwrap();
        let r: usize = b.next().unwrap().trim().parse().unwrap();
        let count = if l == 1 {
            p[r - 1]
        } else {
            p[r - 1] - p[l - 2]
        };
        println!("{}", count)
    }
}
pub fn prefix_sum() {
    /*    range_sum_query(); */
    /*    even_sum_query(); */
    count_vowels();

    //
    //  let mut p = vec![0; n];
    // p[0] = str_line[0];
    // for i in 1..n {
    //     if !matches!(str_line[i], 'a' | 'e' | 'i' | 'o' | 'u') {
    //         p[i] = p[i - 1] + str_line[i]
    //     } else {
    //         p[i] = p[i - 1]
    //     }
    // }
    //  let ans = if l == 1 {
    //     p[r - 1]
    // } else {
    //     p[r - 1] - p[l - 2]
    // };
    // println!("ans")
    //
}
