#![allow(dead_code)]

use std::io;
pub fn two_pointers() {
    //Useing carry Forward Technique
    //
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
        let mut sum = 0;
        while r < n {
            sum += arr[r];
            r += 1;
            print!("{} ", sum);
        }
        println!();
        l += 1;
    }
}
