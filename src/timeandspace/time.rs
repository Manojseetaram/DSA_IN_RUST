#![allow(dead_code)]
use std::io;
pub fn useing_time_com_fact() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n = a.trim().parse().unwrap();
    let mut count = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            print!("{} ", i);
            let j = n / i;
            if i == j {
                count += 1;
            } else {
                count += 2;
            }
        }

        i += 1;
    }
    println!();
    println!("Factors : {}", count)
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
pub fn timeandspace() {
    useing_time_com_fact();
    without_usieng_time_complexity();
}
