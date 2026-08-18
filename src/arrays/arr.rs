#![allow(dead_code)]
use std::{io, mem::swap};

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
//Find th maximum of the arrays
pub fn maximum_number_of_the_array() {
    let a = [1, 2, 3, 6, 4, 99];
    let n = 6;

    let mut b = [0; 6];
    let mut i = 0;
    while i < n {
        b[i] = a[n - i - 1];
        i += 1;
    }
    let mut ans = a[0];
    let mut i = 1;
    let mut location = 1;
    while i < n {
        if a[i] < ans {
            ans = a[i];
            location += 1;
        }

        i += 1;
    }
    println!("ans : {:?} , location :  {}", ans, location);
}
pub fn minmum_elemnt() {
    let a = [11, 2, 3, 4, 5, 6];
    let n = 6;

    let mut ans = a[0];
    let mut location = 1;
    let mut i = 0;
    while i < n {
        if a[i] < ans {
            ans = a[i];
            location += 1;
        }
        i += 1;
    }
    println!("minimum : {} , location : {}", ans, location)
}
pub fn search_in_an_array() {
    let a = [1, 2, 5, 9, 11];
    let n = 5;
    let mut found = false;
    let search = 19;
    let mut i = 0;
    while i < n {
        if a[i] == search {
            found = true;
            break;
        }
        i += 1
    }
    if found {
        println!("Number found ");
    } else {
        println!("Not found")
    }
}
pub fn count_occurences() {
    let a = [1, 2, 3, 4, 1, 1, 1];
    let n = 7;
    let target = 1;
    let mut count = 0;
    let mut i = 0;
    while i < n {
        if a[i] == target {
            count += 1
        }
        i += 1
    }
    println!("{count}")
}
pub fn sorted_array() {
    let a = [1, 2, 3, 4, 4];
    let n = 5;
    let mut flag = false;
    let mut i = 1;
    while i < n {
        if a[i] >= a[i - 1] {
            flag = true;
            break;
        }
        i += 1;
    }
    if flag {
        println!("This is sorted array ")
    } else {
        println!("This is not sorted array")
    }
}

pub fn sort_zero_one() {
    let a = [1, 1, 0, 1, 1, 0];
    let n = 6;
    let mut c0 = 0;
    let mut c1 = 1;
    let mut i = 1;

    while i < n {
        if a[i] == 0 {
            c0 += 1;
        } else {
            c1 += 1;
        }
        i += 1;
    }

    let mut i = 1;
    while i <= c0 {
        print!("0");

        i += 1;
    }
    let mut i = 1;
    while i <= c1 {
        print!("1");
        i += 1;
    }
    println!();
    let target = 0;
    let mut count = 0;
    let mut i = 0;
    while i < n {
        if a[i] == target {
            count += 1;
        }
        i += 1;
    }

    println!("Total number of zeros : {count}");

    let target = 1;
    let mut count = 0;
    let mut i = 0;
    while i < n {
        if a[i] == target {
            count += 1;
        }
        i += 1;
    }
    println!("Total number of one : {count}")
}
pub fn swapa() {
    let mut a = 4;
    let mut b = 7;
    // let temp = a;
    // let a = b;
    // let b = temp;
    swap(&mut a, &mut b);
    println!("a : {a} , b : {b}")
}
pub fn swap_alternate() {
    let mut a = [7, 8, 1, 2, 3, 4, 7];
    let n = 6;
    let mut i = 1;

    while i < n {
        let temp = a[i];
        a[i] = a[i - 1];
        a[i - 1] = temp;
        i += 2;
    }
    println!("{:?}", a);
}
pub fn swap_revers() {
    let mut a = [1, 4, 2, 3, 5, 9, 10];
    let n = 7;
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
        i += 1;
        j -= 1;
    }
    println!("{:?}", a)
}
pub fn missing_number() {
    let a = [2, 1, 9, 1, 2, 3, 12, 3, 9];
    let n = 9;
    let mut i = 0;
    let mut ans = 0;
    let mut flag = false;

    while i < n {
        let target = a[i];
        let mut count = 0;
        let mut j = 0;
        while j < n {
            if target == a[j] {
                count += 1;
            }
            j += 1;
        }
        if count == 1 {
            ans = a[i];
            flag = true;
            break;
        }

        i += 1;
    }
    if flag {
        println!("This is a single number : {:?}", ans);
    } else {
        println!("Number is not found")
    }
}
pub fn missing_numbers() {
    let a = [1, 1, 2, 2, 1];
    let n = 5;
    let mut flag = false;
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let target = a[i];
        let mut count = 0;
        let mut j = 0;
        while j < n {
            if a[j] == target {
                count += 1
            }
            j += 1;
        }
        if count == 1 {
            ans = a[i];
            flag = true;
            break;
        }
        i += 1;
    }
    if flag {
        println!("{ans}")
    } else {
        println!("Not found")
    }
}
pub fn duplicate_number() {
    let a = [0, 7, 2, 5, 4, 7, 0, 1, 3, 6];
    let n = 10;
    let mut i = 0;
    let mut ans = 0;
    let mut flag = false;
    while i < n {
        let mut count = 0;
        let mut j = 0;
        let target = a[i];
        while j < n {
            if target == a[j] {
                count += 1;
            }
            j += 1;
        }
        if count != 1 {
            ans = a[i];
            flag = true;
            break;
        }
        i += 1;
    }
    if flag {
        println!("Number is found : {ans }")
    } else {
        println!("Number is not found ")
    }
}
pub fn print_array_in_reverse() {
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
    let mut i = 0;
    let mut b = vec![0; n];
    while i < n {
        b[i] = arr[n - 1 - i];
        i += 1;
    }
    let mut i = 0;
    while i < n {
        print!("{} ", b[i]);
        i += 1;
    }
    println!();
}
pub fn sum_of_arrayy() {
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
    let mut i = 0;
    let mut sum = 0;

    while i < n {
        sum += arr[i];
        i += 1;
    }
    println!("{sum}")
}
pub fn minimm_element_and_its_postion() {
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
    let mut i = 1;
    let mut min = arr[0];
    let mut position = 1;

    while i < n {
        if arr[i] < min {
            min = arr[i];
            position = i + 1;
        }
        i += 1;
    }
    println!("{} {}", min, position);
}
pub fn maximum_element_and_its_position() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    if n == 0 {
        println!("0");
        return;
    }
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i128> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut maximum: i128 = arr[0];
    let mut position: usize = 1;
    let mut i = 0;

    while i < n {
        if arr[i] > maximum {
            maximum = arr[i];
            position = i + 1
        }

        i += 1;
    }
    println!("{} {}", maximum, position);
}
pub fn search_elemen_in_array() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut input = a.trim().split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();
    let m: i64 = input.next().unwrap().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let mut flag = false;
    let mut i = 0;
    while i < n {
        if arr[i] == m {
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
pub fn count_occurrences() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut input = a.trim().split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();
    let m: i64 = input.next().unwrap().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    let mut i = 0;
    let mut count = 0;
    while i < n {
        if arr[i] == m {
            count += 1;
        }
        i += 1;
    }
    println!("{count}")
}
pub fn array_is_sorted() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    let mut i = 1;
    let mut flag = true;
    while i < n {
        if arr[i] < arr[i - 1] {
            flag = false;
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
pub fn alpha() {
    let n = 8;

    let mut i = 0;

    while i < n {
        let ch = (b'A' + i as u8) as char;
        println!("{}", ch);
        i += 1;
    }
}
pub fn sort() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let t: usize = a.trim().parse().unwrap();
    for _ in 0..t {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let n: usize = a.trim().parse().unwrap();
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut arr: Vec<i64> = a
            .trim()
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n {
                if arr[i] > arr[j] {
                    arr.swap(i, j);
                }
                j += 1;
            }
            i += 1;
        }
        let mut i = 0;
        while i < n {
            print!("{} ", arr[i]);
            i += 1;
        }
        println!();
    }
}
pub fn reverse() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let mut arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;
    let mut j = n - 1;

    while i < j {
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i += 1;
        j -= 1;
    }
    let mut i = 0;
    while i < n {
        print!("{} ", arr[i]);
        i += 1;
    }
    println!();
}
pub fn arrange_the_numbers() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    for _ in 0..n {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let t: usize = a.trim().parse().unwrap();
        let mut arr = vec![0; t];

        let mut i = 0;
        let mut j = t - 1;
        let mut num = 1;
        let mut fill_left = true;

        while i <= j {
            if fill_left {
                arr[i] = num;
                i += 1;
            } else {
                arr[j] = num;
                j -= 1;
            }
            num += 1;
            fill_left = !fill_left;
        }
        let mut i = 0;

        while i < t {
            print!("{} ", arr[i]);
            i += 1;
        }
        println!();
    }
}
pub fn swap_alternates() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    for _ in 0..n {
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let t: usize = a.trim().parse().unwrap();
        a.clear();
        io::stdin().read_line(&mut a).unwrap();
        let mut arr: Vec<i128> = a
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut i = 0;

        while i <= t {
            if i < t - 1 {
                let temp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = temp;
            }
            i += 2;
        }
        let mut i = 0;
        while i < t {
            print!("{} ", arr[i]);
            i += 1;
        }
        println!();
    }
}
pub fn arrays() {
    /*   input_output(); */
    // reverse_array();
    // two_pointer_reverse_array();
    // two_poiter_char_array();
    /*  some_of_array(); */
    /*     some_of_array_usieng_two_pointers(); */
    /*  maximum_number_of_the_array(); */
    /*     minmum_elemnt(); */
    /* search_in_an_array(); */
    /*     count_occurences(); */
    /*  sorted_array(); */
    /*     sort_zero_one(); */
    /* swapa(); */
    /* swap_revers(); */
    /*     missing_numbers(); */
    /*     duplicate_number(); */
    /*    print_array_in_reverse(); */
    /*     sum_of_arrayy(); */
    /*  minimm_element_and_its_postion(); */
    /*   maximum_element_and_its_position(); */
    /*     count_occurrences(); */
    /* array_is_sorted(); */
    /*     alpha(); */
    /*   array_is_sorted(); */
    /*  sort(); */
    /*     maximum_element_and_its_position(); */
    /*     arrange_the_numbers(); */
    swap_alternates();
}
