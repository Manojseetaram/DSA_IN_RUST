//When you want repeat an action again and agian
//till a certain condition we can use loops

//Start Problem Statement

//print 1 to 100
#![allow(dead_code)]
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
//Last digit off number
pub fn last_digit_of_number(n: u32) {
    let i = n;
    println!("{}", i % 10);
}
//Print number in reverse
pub fn print_number_in_reverse(n: u32) {
    let mut temp = n;
    while temp != 0 {
        println!(" reverse : {}", temp % 10);
        temp /= 10
    }
    //Output
    //3
    //2
    //1
    // Create actual reversed number
    let mut temp = n;
    let mut rev = 0;

    while temp != 0 {
        let digit = temp % 10;
        rev = rev * 10 + digit;
        temp /= 10;
    }

    println!("Reversed number: {rev}");
    //Output : Reversed number : 321

    //Useing the for loop conevr to int to string
    let mut a = String::new();
    for digit in n.to_string().chars().rev() {
        a.push(digit);
    }
    println!("For loop reverse : {a}");
    //Out put : 321
    let mut r = 0;
    for digit in n.to_string().chars().rev() {
        r = r * 10 + digit.to_digit(10).unwrap();
    }
    println!("{r}");
    //Output : 321

    //Useing loop
    let mut rev = 0;
    let mut num = n;
    loop {
        if num == 0 {
            break;
        }
        let digit = num % 10;
        rev = rev * 10 + digit;
        num /= 10
    }
    println!("Loops : {rev}");
    //Output : Loops : 321
}

//DIGITS SUM
pub fn digits_sum(n: u32) {
    //Useing while loop
    let mut num = n;
    let mut r = 0;
    while num != 0 {
        let digit = num % 10;
        r = r + digit;
        num /= 10
    }
    println!("{r}");

    //Useing For loop
    let mut num = 0;
    for digit in n.to_string().chars().rev() {
        num = num + digit.to_digit(10).unwrap()
    }
    println!("{num}");

    //Using  loop
    let mut rev = 0;
    let mut num = n;
    loop {
        if num == 0 {
            break;
        }
        let digit = num % 10;
        rev = rev + digit;
        num /= 10
    }
    println!("{rev}");
    //Output : 6
}
//Reverse and store in a vatiable ;
pub fn rev_and_store_in_var(n: u32) -> u32 {
    let mut ans = 0;
    let mut num = n;

    while num != 0 {
        ans = ans * 10 + num % 10;
        num /= 10
    }
    ans
}
//PALINDROME
pub fn palindrome(n: u32) -> bool {
    //While loop
    let mut rev = 0;
    let mut num = n;
    println!("inpur : {num }");
    while num != 0 {
        let digit = num % 10;
        rev = rev * 10 + digit;
        num /= 10
    }
    println!("sharma : {rev}");
    if rev == n { true } else { false }
    //Output  : 121 : true
}
pub fn palindrome_for_loop(n: u32) -> bool {
    //For loop
    let mut rev = 0;
    for digit in n.to_string().chars().rev() {
        rev = rev * 10 + digit.to_digit(10).unwrap()
    }
    if rev == n { true } else { false }
}
pub fn palindrome_n(n: i32) -> bool {
    let mut i = n;
    let mut sum = 0;

    while i != 0 {
        sum = sum * 10 + i % 10;
        i = i / 10;
    }

    if sum == n { true } else { false }
}
pub fn factorial_number(n: i32) {
    let mut fact = 1;
    let mut i = n;
    while i != 0 {
        fact = fact * i;

        i -= 1;
    }
    println!("{fact}");
}
pub fn loops_function() {
    // print_one_to_hundred();
    // print_one_to_n_numbers(10);
    // print_l_to_r_numbers(4, 10);
    // even_numbers(1, 10);
    // albhabets_a_to_z();
    // multiplication_table(10, 10);
    // last_digit_of_number(1297);
    // print_number_in_reverse(123);
    // digits_sum(111111);
    // println!("{}", rev_and_store_in_var(321));
    // println!("{}", palindrome(121));
    // println!("{}", palindrome_for_loop(212));
    // println!("{}", palindrome_n(121))
    /*     factorial_number(5); */
}
