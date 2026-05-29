//When you want repeat an action again and agian
//till a certain condition we can use loops

//Start Problem Statement

//print 1 to 100

pub fn print_one_to_hundred() {
    //for loop
    for i in 1..=5 {
        println!("{i:#?}")
    }

    //While loop
    let mut n = 5;
    while n >= 1 {
        println!("{n:?}");
        n -= 1;
    }

    //infinate loop
    let mut num = 0;
    loop {
        num += 1;
        println!("{num:?}");

        if num == 5 {
            break;
        }
    }
    //Output : 1 , 2 , 3, 4, 5
}
pub fn print_one_to_n_numbers(n: u32) {
    //Printing N numbers useing for loop
    for i in 1..=n {
        println!("{i:?}");
    }
    //While loop
    let mut num = n;
    while num >= 1 {
        println!("{num:?}");
        num -= 1
    }

    let mut count = 0;
    loop {
        count += 1;
        println!("{count:?}");
        if count == n {
            break;
        }
    }
    //Output : 1 to 10
}
//Print L to R numbers
pub fn print_l_to_r_numbers(l: u32, r: u32) {
    //for loop
    for i in l..=r {
        println!("{i:?}");
    }
    //While loop
    let mut num = r;
    while num >= l {
        println!("{num:?}");

        num -= 1
    }
    //Loops
    let mut count = l;
    loop {
        println!("{count:?}");
        if count == r {
            break;
        }
        count += 1;
    }
}

//All even number of one to 10
pub fn even_numbers(l: u32, r: u32) {
    let mut count = l;
    loop {
        if count > r {
            break;
        }
        if count % 2 == 0 {
            println!("even : {count}");
        }
        count += 1;
    }

    for i in l..=r {
        if i % 2 == 0 {
            println!("even  : {i:?}")
        }
    }

    let mut num = r;
    while num >= l {
        if num > r {
            break;
        }
        if num % 2 == 0 {
            println!("even : {num:?}");
        }
        num -= 1;
    }
}

//Prinitng A to z Alphbets
pub fn albhabets_a_to_z() {
    //for loop
    for i in 'A'..='Z' {
        println!("{:#?}", i);
    }
    //while loop
    let mut ch = 'Z' as u8;
    while ch >= 'A' as u8 {
        println!("{:?}", ch as char);

        if ch > 'Z' as u8 {
            break;
        }
        ch -= 1
    }
    //Loops
    let mut c = 'A' as u8;
    loop {
        println!("{:?}", c as char);
        if c == 'Z' as u8 {
            break;
        }
        c += 1;
    }
}
//print multiplication table
pub fn multiplication_table(n: u32, limit: u32) {
    let mut i = 1;
    while i <= limit {
        println!("{n} x {i} = {}", n * i);
        i += 1;
    }

    for i in 1..=limit {
        println!("{n} x {i} = {}", n * i)
    }
    let mut a = 1;
    loop {
        if a > limit {
            break;
        }
        println!("{n} x {a} = {}", n * a);

        a += 1;
    }
}
pub fn last_digit_of_number(n: u32) {
    let i = n;
    println!("{}", i % 10);
}
