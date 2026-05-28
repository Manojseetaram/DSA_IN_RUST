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
        if num == 6 {
            break;
        }
        println!("{num:?}");
    }
    //Output : 1 , 2 , 3, 4, 5
}
