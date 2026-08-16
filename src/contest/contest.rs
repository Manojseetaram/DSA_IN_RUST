#![allow(dead_code)]
use std::io;

pub fn count_zeros() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    let mut chars = a.trim().chars();
    let mut count = 0;
    while let Some(ch) = chars.next() {
        if ch == '0' {
            count += 1;
        }
    }
    println!("{count}");
}

pub fn hello_codeforce_n_times() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        println!("Hello Codeforces {i}");
        i += 1;
    }
}
pub fn is_vowel() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: char = a.trim().chars().next().unwrap();
    if "aieouAIEOU".contains(n) {
        println!("YES");
    } else {
        println!("NO");
    }
}
pub fn second_last_digit() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut ans = 0;
    let mut i = 1;
    while i <= n {
        ans = (n / 10) % 10;
        i += 1;
    }
    println!("{ans}")
}
pub fn leap_year() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    if n % 400 == 0 || (n % 4 == 0 && n % 100 != 0) {
        println!("Yes")
    } else {
        println!("No")
    }
}
pub fn count_good_numbers() {
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
    let mut count = 0;

    for i in 0..n {
        let x = arr[i];

        if (x != 0 && 18 % x == 0) || x % 45 == 0 {
            count += 1;
        }
    }

    println!("{count}");
}
pub fn fizz_buzz() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        if i % 5 == 0 && i % 3 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }

        i += 1;
    }
}
pub fn empty_rectangle() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut k = a.trim().split_whitespace();
    let n: usize = k.next().unwrap().parse().unwrap();
    let m: usize = k.next().unwrap().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut j = 1;
        while j <= m {
            if i == 1 || i == n || j == 1 || j == m {
                print!("^");
            } else {
                print!(" ")
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}
pub fn shifted_pramid() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut j = 1;
        while j <= i - 1 {
            print!(" ");
            j += 1;
        }

        let mut j = 1;
        while j <= i {
            print!("x");
            j += 1;
        }
        println!();
        i += 1;
    }
}
pub fn hourglass() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = n;
    while i >= 2 {
        let mut j = 1;
        while j <= n - i {
            print!(" ");
            j += 1;
        }
        let mut j = 1;
        while j <= i {
            if j == i {
                print!(".");
            } else {
                print!(". ")
            }
            j += 1;
        }
        println!();
        i -= 1;
    }

    let mut i = 1;
    while i <= n {
        let mut j = 1;
        while j <= n - i {
            print!(" ");
            j += 1;
        }
        let mut j = 1;
        while j <= i {
            if j == i {
                print!(".");
            } else {
                print!(". ")
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}
pub fn arrow() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    let mut i = 1;
    while i <= n {
        let mut j = 1;
        while j <= i - 1 {
            print!(" ");
            j += 1;
        }
        let mut j = 1;
        while j <= i {
            if j == 1 || j == i {
                print!("> ");
            } else {
                print!("  ")
            }

            j += 1;
        }
        println!();
        i += 1;
    }
    let mut i = n - 1;
    while i >= 1 {
        let mut j = 1;
        while j <= i - 1 {
            print!(" ");
            j += 1;
        }

        let mut j = 1;
        while j <= i {
            if j == 1 || j == i {
                print!("> ");
            } else {
                print!("  ")
            }

            j += 1;
        }
        println!();
        i -= 1;
    }
}
pub fn contest() {
    /*   count_zeros(); */
    /*     hello_codeforce_n_times(); */
    /* second_last_digit(); */
    /*  leap_year(); */
    /*   count_good_numbers(); */
    /*     fizz_buzz(); */
    /*     empty_rectangle(); */
    /*     shifted_pramid(); */
    /* hourglass(); */
    /*    arrow(); */
}
