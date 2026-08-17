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

    // Condition 1: Numbers less than or equal to 1 are not prime
    if n <= 1 {
        println!("Not Prime");
        return;
    }

    let mut i = 2; // Start checking from 2, not 1
    let mut is_prime = true; // Assume it is prime until proven otherwise

    // Condition 2: Only loop while i * i <= n (the square root condition)
    while i * i <= n {
        if n % i == 0 {
            is_prime = false; // Found a factor, so it's not prime
            break;
        }
        i += 1;
    }

    if is_prime {
        println!("Prime");
    } else {
        println!("Not Prime");
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
pub fn print_primes_from_one_to_n() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 2;
    while i <= n {
        let mut j = 2;
        let mut is_prime = true;
        while j * j <= i {
            if i % j == 0 {
                is_prime = false;
                break;
            }

            j += 1;
        }
        if is_prime {
            print!("{i} ");
        }
        i += 1;
    }
    println!();
}
pub fn ncr() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();

    let mut input = a.trim().split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let r: usize = input.next().unwrap().parse().unwrap();

    let mut fact_n = 1;
    let mut i = 1;
    while i <= n {
        fact_n *= i;
        i += 1;
    }
    let mut facr_r = 1;
    let mut i = 1;
    while i <= r {
        facr_r *= i;
        i += 1;
    }
    let mut fact_n_r = 1;
    let mut i = 1;
    while i <= n - r {
        fact_n_r *= i;
        i += 1;
    }
    let ans = fact_n / (facr_r * fact_n_r);
    println!("{ans}");
}
pub fn find_hcf() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();

    let mut input = a.trim().split_whitespace();

    let mut a: usize = input.next().unwrap().parse().unwrap();
    let mut b: usize = input.next().unwrap().parse().unwrap();

    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    println!("{a}");
}
pub fn prime() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: i64 = a.trim().parse().unwrap();
    if n <= 1 {
        println!("Not Prime");
        return;
    }
    let mut i = 2;
    let mut flag = true;
    while i < n {
        if n % i == 0 {
            flag = false;
            break;
        }
        i += 1;
    }
    if flag {
        println!("Prime")
    } else {
        println!("Not Prime")
    }
}
pub fn function() {
    /*     hello_function(); */
    /*   print_factors(); */
    /*     print_factors_decresing_order(); */
    /*     check_prime(); */
    /*   check_prime(); */
    /* factorial(); */
    /*     ncr(); */
    /*     print_primes_from_one_to_n(); */
    /*     l_to_r(); */
    /*    ncr(); */
    /* find_hcf(); */
    prime();
}
