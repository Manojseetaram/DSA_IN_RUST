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
pub fn sum_of_all_sub_arraty() {
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
    p[0] = arr[0];
    for i in 1..n {
        p[i] = p[i - 1] + arr[i]
    }
    let mut ans = 0i64;

    for i in 0..n {
        ans += arr[i] * (i as i64 + 1) * (n as i64 - i as i64);
        print!("{ans}")
    }
}

pub fn sum_of_all_sub_array_separatly() {
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
            let mut sum = 0;
            while i <= r {
                sum += arr[i];

                i += 1;
            }
            println!("{sum}");

            r += 1;
        }

        l += 1;
    }
}

pub fn sum_of_all_sub_array_separatly_medium() {
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
    p[0] = arr[0];
    for i in 1..n {
        p[i] = p[i - 1] + arr[i];
    }
    let mut l = 0;

    while l < n {
        let mut r = l;
        let mut sum;
        while r < n {
            if l == 0 {
                sum = p[r]
            } else {
                sum = p[r] - p[l - 1]
            }

            println!("{sum}");
            r += 1;
        }

        l += 1;
    }
}
pub fn sum_of_all_sub_array_separatly_hard() {
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
    p[0] = arr[0];
    for i in 1..n {
        p[i] = p[i - 1] + arr[i];
    }

    for l in 0..n {
        for r in l..n {
            let sum = if l == 0 { p[r] } else { p[r] - p[l - 1] };
            println!("{sum}");
        }
    }
}

pub fn subarray() {
    /*     print_all_subarray(); */
    /*     sum_of_all_sub_arraty(); */
    /*   sum_of_all_sub_array_separatly_hard(); */
}
