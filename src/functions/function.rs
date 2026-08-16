#![allow(dead_code)]

use std::io;

pub fn hello_function() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let count = "I am learning functions";

        println!("{count}");
        i += 1;
    }
}
pub fn print_factors() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        if n % i == 0 {
            print!("{i} ");
        }

        i += 1;
    }
    println!();
}
pub fn print_factors_decresing_order() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = n;
    while i >= 1 {
        if n % i == 0 {
            print!("{} ", i);
        }

        i -= 1;
    }
    println!()
}
pub fn check_prime() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: i64 = a.trim().parse().unwrap();
    let mut i = 1;
    let mut flag = false;

    while i <= n {
        if n % 2 == 0 {
            flag = true;
            break;
        }
        i += 1;
    }
    if !flag {
        println!("Prime")
    } else {
        println!("Not Prime")
    }
}
pub fn factorial() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    let mut fact = 1;
    while i <= n {
        fact *= i;

        i += 1;
    }
    println!("{fact}");
}
pub fn ncr() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n = a.trim().split_whitespace();
    let k: usize = n.next().unwrap().parse().unwrap();
    let r: usize = n.next().unwrap().parse().unwrap();
}
pub fn function() {
    /*     hello_function(); */
    /*   print_factors(); */
    /*     print_factors_decresing_order(); */
    /*     check_prime(); */
    /*     check_prime(); */
    /* factorial(); */
    ncr();
}
