#![allow(dead_code)]
use std::{char, io};
pub fn check_same() {
    let ch1 = "aa";
    let ch2 = "aa";
    if ch2 == ch1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
pub fn ascii() {
    let ch = 'a' as i32;
    println!("{ch}");
    let ch1 = 'A' as i32;
    println!("{ch1}");
    let ch2 = '0' as i32;
    println!("{ch2}");
    let mut a = 'A' as i32;
    if a >= 'A' as i32 && a <= 'Z' as i32 {
        a += 32
    }
    println!("{:?}", a as u8 as char);
}
pub fn charss() {
    // let s = "manoj";
    // let chars: Vec<char> = s.chars().collect();
    //
    // let mut i = 0;
    // while i < chars.len() {
    //     println!("{}", chars[i]);
    //     i += 1;
    // }
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let word = a.split_whitespace().next().unwrap();
    println!("{}", word)
}
pub fn brothers() {
    let mut a1 = String::new();
    io::stdin().read_line(&mut a1).unwrap();
    let mut t1 = String::new();
    io::stdin().read_line(&mut t1).unwrap();
    let mut a2 = String::new();
    io::stdin().read_line(&mut a2).unwrap();

    let mut t2 = String::new();
    io::stdin().read_line(&mut t2).unwrap();
    if t1 == t2 {
        println!("They are brothers");
    } else {
        println!("Not brothers");
    }
}
pub fn lexicographical_order() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    if a < b {
        println!("A");
    } else if a == b {
        println!("Equal")
    } else {
        println!("B")
    }
}

pub fn replace_character() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).unwrap();
    let mut b2 = String::new();
    io::stdin().read_line(&mut b2).unwrap();

    /*   let mut iter = b1.split_whitespace(); */
    let mut n: Vec<char> = a.trim().chars().collect();
    let mut i = 0;
    // let old = iter.next().unwrap().chars().next().unwrap();
    // let new = iter.next().unwrap().chars().next().unwrap();
    let old = b1.trim().chars().next().unwrap();
    let new = b2.trim().chars().next().unwrap();
    while i < n.len() {
        if n[i] == old {
            n[i] = new
        }
        i += 1;
    }
    let ans: String = n.into_iter().collect();
    println!("{}", ans);
}
pub fn string_palindrome() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n: Vec<char> = a.trim().chars().collect();
    let original = n.clone();
    let mut i = 0;
    let mut j = n.len() - 1;
    while i < j {
        let tmp = n[i];
        n[i] = n[j];
        n[j] = tmp;
        i += 1;
        j -= 1;
    }
    if original == n {
        println!("yes")
    } else {
        println!("No")
    }
}
pub fn count_words() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: Vec<char> = a.chars().collect();
    let mut count = 0;
    let mut i = 0;

    while i < n.len() {
        if n[i] == ' ' {
            count += 1;
        }
        i += 1;
    }
    println!("{}", count + 1);
}

pub fn palindrome() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n: Vec<char> = a.trim().chars().collect();
    let collect = n.clone();

    let mut i = 0;
    let mut j = n.len() - 1;
    while i < j {
        let tmp = n[i];
        n[i] = n[j];
        n[j] = tmp;
        i += 1;
        j -= 1;
    }
    if collect == n {
        println!("YES")
    } else {
        println!("No")
    }
}
pub fn reverse() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut n: Vec<char> = a.trim().chars().collect();
    let mut i = 0;
    let mut j = n.len() - 1;
    while i < j {
        let tmp = n[i];
        n[i] = n[j];
        n[j] = tmp;
        i += 1;
        j -= 1;
    }
    let ans: String = n.into_iter().collect();
    println!("{}", ans);
}

pub fn trim_space() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let trim_space: String = a.trim().split_whitespace().collect();
    println!("{}", trim_space);
}
pub fn remove_character() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).unwrap();

    let old = b1.trim().chars().next().unwrap();

    let mut n: Vec<char> = a.trim().chars().collect();
    let mut i = 0;
    while i < n.len() {
        if n[i] == old {
            n[i] = ' '
        }
        i += 1;
    }
    let ans: String = n.into_iter().collect();
    let ans1: String = ans.split_whitespace().collect();
    println!("{}", ans1);
}
pub fn lexicographical_orders() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    if a < b {
        println!("A")
    } else if a == b {
        println!("Equal")
    } else {
        println!("B")
    }
}
pub fn strings() {
    /*     check_same(); */
    // ascii();
    // charss();
    /*     brothers(); */
    /*     lexicographical_order(); */
    /*     replace_character(); */
    /*   string_palindrome(); */
    /*  count_words(); */
    /*     reverse(); */
    /*     trim_space(); */
    /*     remove_character(); */
    lexicographical_orders();
}
