#![allow(dead_code)]

use std::io;

pub fn carry_forword_method() {
    //Useing carry Forward Technique
    //
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n = a.trim().split_whitespace();
    let t: usize = n.next().unwrap().parse().unwrap();
    let z: usize = n.next().unwrap().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut l = 0;
    while l < t {
        let mut r = l;
        let mut sum = 0;
        while r < t {
            sum += arr[r];
            r += 1;
            if t == z {}
            print!("{} ", sum);
        }
        println!();
        l += 1;
    }
}
pub fn sum() {
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
            let mut sum = 0;
            let mut i = l;

            while i <= r {
                sum += arr[i];

                i += 1;
            }
            print!("{} ", sum);
            r += 1;
        }
        println!();
        l += 1;
    }
}
pub fn max_subarray_size_k() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n = a.trim().split_whitespace();
    let n1: usize = n.next().unwrap().parse().unwrap();
    let k: usize = n.next().unwrap().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut ans = i64::MIN;
    let mut l = 0;
    while l < n1 {
        let mut r = l;
        let mut sum = 0;

        while r < n1 {
            sum += arr[r];
            if r - l + 1 == k {
                ans = ans.max(sum);
            }

            r += 1;
        }

        l += 1;
    }
    println!("{}", ans);
}
pub fn max_subarray_size_k_with_time() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n = a.trim().split_whitespace();
    let n1: usize = n.next().unwrap().parse().unwrap();
    let k: usize = n.next().unwrap().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut sum = 0;
    let mut l = 0;
    while l < k {
        sum += arr[l];
        l += 1;
    }
    let mut ans = sum;
    let mut i = k;
    while i < n1 {
        sum += arr[i];
        sum -= arr[i - k];
        ans = ans.max(sum);
        i += 1;
    }
    println!("{}", ans)
}

pub fn two_pointers() {
    /*     carry_forword_method(); */
    /*     sum(); */
    /*     max_subarray_size_k(); */
    max_subarray_size_k_with_time();
}
