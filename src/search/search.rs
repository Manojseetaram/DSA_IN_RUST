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
pub fn serching() {
    binary_search();
}
