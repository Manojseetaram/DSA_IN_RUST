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
    sort_zero_one();
}

