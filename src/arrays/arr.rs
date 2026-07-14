#![allow(dead_code)]
use std::io;

pub fn input_output() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut a = vec![0; n];

    let mut i = 0;
    while i < n {
        a[i] = nums[i];
        i += 1;
    }

    let mut i = 0;
    while i < n {
        print!("{} ", a[i]);
        i += 1;
    }
}
pub fn some_of_array_usieng_two_pointers() {
    let mut a = [1, 2, 3, 4, 5, 6];
    let n = 6;
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
        i += 1;
        j -= 1;
    }
    let mut some = 0;
    let mut i = 0;
    while i <= n - 1 {
        some += a[i];
        i += 1
    }
    println!("{:?}", some)
}
pub fn reverse_array() {
    let a = [11, 24, 87, 9];
    let n = 4;

    let mut b = [0; 4];

    let mut i = 0;
    while i <= n - 1 {
        b[i] = a[n - 1 - i];
        i += 1;
    }

    let mut i = 0;
    while i <= n - 1 {
        print!("{} ", b[i]);
        i += 1;
    }

    println!();
}
pub fn some_of_array() {
    let a = [10, 4, 4, 2];
    let n = 4;
    let mut b = [0; 4];
    let mut i = 0;

    while i <= n - 1 {
        b[i] = a[n - 1 - i];
        i += 1
    }
    let mut sum = 0;
    let mut i = 0;
    while i <= n - 1 {
        sum += b[i];
        i += 1
    }
    println!("{:?}", sum);
}
pub fn two_pointer_reverse_array() {
    let mut a = [10, 4, 4, 2];
    let n = 4;

    let mut i = 0;
    let mut j = n - 1;

    while i < j {
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;

        i += 1;
        j -= 1;
    }

    let mut k = 0;
    while k < n {
        print!("{} ", a[k]);
        k += 1;
    }
    println!()
}

pub fn two_poiter_char_array() {
    let mut a = ["h", "e", "l", "l", "o"];
    let n = 5;
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
        i += 1;
        j -= 1;
    }

    print!("{:? }", a);

    println!()
}
pub fn arrays() {
    /*   input_output(); */
    // reverse_array();
    // two_pointer_reverse_array();
    // two_poiter_char_array();
    /*  some_of_array(); */
    /*     some_of_array_usieng_two_pointers(); */
    let a = [1, 3, 4, 1];
    let n = 4;
    let mut b = [0; 4];
    let mut i = 0;
    while i < n {
        b[i] = a[n - i - 1];
        i += 1
    }
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        sum += a[i];
        i += 1;
    }
    println!("{sum}")
}
