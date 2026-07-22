#![allow(dead_code)]
use std::io;
pub fn useing_time_com_fact() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();

    let mut count = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            let j = n / i;
            if i == j {
                count += 1;
            } else {
                count += 2;
            }
        }

        i += 1;
    }

    println!("{}", count)
}
pub fn without_usieng_time_complexity() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n = a.trim().parse().unwrap();
    let mut i = 1;
    let mut count = 0;
    while i <= n {
        if n % i == 0 {
            count += 1;
            print!("{} ", i)
        }

        i += 1;
    }
    println!();
    println!("Factors : {}", count)
}
pub fn natural_sum() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        i += 1;
    }
    println!("{}", sum)
}
pub fn interval_sum() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let t = a.trim().parse().unwrap();
    for _ in 0..t {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut n = a.trim().split_whitespace();

        let z: usize = n.next().unwrap().trim().parse().unwrap();
        let m: usize = n.next().unwrap().trim().parse().unwrap();

        let count = (m - z) + 1;

        let sum = (m + z) * count / 2;
        println!("{}", sum)
    }
}
pub fn counting_intervals() {}
pub fn timeandspace() {
    /*    useing_time_com_fact(); */
    /*  without_usieng_time_complexity(); */
    /*     natural_sum(); */
    interval_sum();
}
