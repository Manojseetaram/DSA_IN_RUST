#![allow(dead_code)]
use std::io;
pub fn binary_search() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i32> = a.split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let target: i32 = a.trim().parse().unwrap();
    let mut flag = false;
    let mut l = 0;
    let mut r = n - 1;
    while l <= r {
        let mid = (l + r) / 2;
        if arr[mid] == target {
            flag = true;
            break;
        } else if arr[mid] > target {
            r = mid - 1
        } else {
            l = mid + 1
        }
    }
    if flag {
        println!("YES")
    } else {
        println!("NO")
    }
}
pub fn words_binary_search() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut names: Vec<String> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        names.push(input.trim().to_string());
    }
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let target = input.trim().to_string();
    let mut l = 0;
    let mut r = n - 1;
    let mut flag = false;
    while l <= r {
        let mid = (l + r) / 2;
        if names[mid] == target {
            flag = true;
            break;
        } else if names[mid] > target {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    if flag {
        println!("YES")
    } else {
        println!("NO")
    }
}
pub fn linear_serch() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i32> = a.split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let target: i32 = a.trim().parse().unwrap();
    let mut flag = false;
    let mut i = 0;
    while i < n {
        if arr[i] == target {
            flag = true;
            break;
        }
        i += 1;
    }
    if flag {
        println!("YES")
    } else {
        println!("NO")
    }
}
pub fn serching() {
    /*  binary_search(); */
    /*     words_binary_search(); */
    linear_serch();
}
