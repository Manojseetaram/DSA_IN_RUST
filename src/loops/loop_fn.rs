//When you want repeat an action again and agian
//till a certain condition we can use loops

//Start Problem Statement

//print 1 to 100

pub fn print_one_to_hundred() {
    //for loop
    for i in 1..=5.into() {
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
}
