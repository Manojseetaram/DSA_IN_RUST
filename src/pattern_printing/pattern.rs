#![allow(dead_code)]
pub fn one_to_n_number(n: u32) {
    for i in 1..=n {
        println!("{i:?}");
    }
}
//Printin star like a
//*
//**
//***
//****
//*****
pub fn one_to_n_number_star(n: u32) {
    for i in 1..=n {
        println!("{}", "*".repeat(i as usize));
    }
}
pub fn one_to_n_number_of_star(n: u32) {
    for _ in 1..=n {
        println!("**");
    }
}
pub fn one_to_n_number_of_four_star(n: u32) {
    for _ in 1..=n {
        println!("{}", "*".repeat(4));
    }
}
//Print the M stars in evry row
pub fn m_star_print_evry_row(m: u32, n: u32) {
    for _ in 1..=n {
        println!("{}", "*".repeat(m.try_into().unwrap()));
    }
}
//Print SQUARE
//Given n (no. of rows and cols ), print the following pattern:
pub fn sqare_star_print(m: u32) {
    for _ in 1..=m {
        println!("{}", "*".repeat(m.try_into().unwrap()))
    }
}
//pYRAMID
//Given n(no. of rows), print the following pattern ;
pub fn pyramid_print(n: u32) {
    for i in 1..=n {
        println!("{}", "*".repeat(i as usize))
    }
}
//Reverse Pyramid
pub fn reverse_pyramid(n: u32) {
    for i in (1..=n).rev() {
        println!("{}", "*".repeat(i as usize))
    }
}
//Hollow Square
//Given n (no .of rows and cols ) , print the following pattern
pub fn hollow_square(m: u32) {
    for row in 1..=m {
        for column in 1..=m {
            if row == m || row == 1 || column == 1 || column == m {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
//Print Hollow squre
pub fn hollow_sqaure_useing_m_and_n(n: u32, m: u32) {
    for row in 1..=n {
        for column in 1..=m {
            if row == 1 || row == n || column == 1 || column == m {
                print!("*")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}
//Hollow Pyramid
pub fn hollow_pyramid(n: u32) {
    for row in 1..=n {
        for column in 1..=n {
            if row == n || column == 1 || column == row {
                print!("*")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}
//Hollow revrse pyramid
pub fn hollow_rev_pyramid(n: u32) {
    for row in (1..=n).rev() {
        for column in (1..=n).rev() {
            if row == n || column == 1 || row == column {
                print!("*")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}
//Nuberd Rectangle
pub fn numbered_rectangle(n: u32) {
    for i in 1..=n {
        for _ in 1..=n {
            print!("{i}")
        }
        println!()
    }
}
//Numbered rectagnle lets n and m value
pub fn numeric_rectanagle(n: u32, m: u32) {
    for abc in 1..=n {
        for _ in 1..=m {
            print!("{abc}")
        }
        println!()
    }
}
//ABC rectangle
pub fn abc_reactangle(n: u32, m: u32) {
    for i in 0..n {
        let ch = (b'A' + i as u8) as char;
        for _ in 0..m {
            print!("{ch}")
        }
        println!()
    }
}
pub fn abc_rectangle_diff(n: u32, m: u32) {
    for _ in 0..n {
        for i in 0..m {
            let ch = (b'A' + i as u8) as char;
            print!("{ch}")
        }
        println!()
    }
}

//Binary pyramid
pub fn binary_pyramid(n: i32) {
    let mut i = 1;
    while i <= n {
        let mut j = 1;
        let mut num;
        if i % 2 == 0 {
            num = 1;
        } else {
            num = 0;
        }
        while j <= i {
            print!("{num}");
            if num == 0 {
                num = 1;
            } else {
                num = 0
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}

//Traingel pattern
//we shoul make this n - 1
pub fn hollow_traingel(n: i32) {
    let mut i = 1;
    while i <= n {
        let mut j = 1;
        while j <= i {
            if j == 1 || j == n || j == i {
                print!("*");
            } else {
                print!(" ")
            }
            j += 1;
        }
        println!();
        i += 1;
    }
    let mut i = n - 1;
    while i >= 1 {
        let mut j = 1;
        while j <= i {
            if j == 1 || i == j {
                print!("*");
            } else {
                print!(" ")
            }
            j += 1;
        }
        println!();
        i -= 1;
    }
}
pub fn pattern_printing() {
    // one_to_n_number(5);
    // one_to_n_number_star(6);
    // one_to_n_number_of_star(5);
    // one_to_n_number_of_four_star(5);
    // m_star_print_evry_row(1, 5);
    // sqare_star_print(5);
    // pyramid_print(8);
    // reverse_pyramid(12);
    // hollow_square(10);
    // hollow_sqaure_useing_m_and_n(10, 4);
    // hollow_pyramid(10);
    // hollow_rev_pyramid(10);
    // numbered_rectangle(5);
    // numeric_rectanagle(10, 3);
    // abc_reactangle(10, 6);
    // abc_rectangle_diff(10, 6)
    /*     binary_pyramid(5); */
    hollow_traingel(5);
}
