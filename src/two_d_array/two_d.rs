#![allow(dead_code)]
pub fn row_to_col() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    while i < r {
        let mut j = 0;
        while j < c {
            print!("{}  ", a[i][j]);
            j += 1;
        }
        println!();
        i += 1;
    }
}
pub fn col_to_row() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    while i < c {
        let mut j = 0;
        while j < r {
            print!("{}  ", a[j][i]);
            j += 1;
        }
        println!();
        i += 1;
    }
}
pub fn wave_print_row_wise() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    while i < r {
        if i % 2 == 0 {
            let mut j = 0;

            while j < c {
                print!("{}  ", a[i][j]);
                j += 1;
            }
        } else {
            // let mut j = (c - 1) as i32;
            // while j >= 0 {
            //     print!("{}  ", a[i][j as usize]);
            //     j -= 1;
            // }
            let mut j = c - 1;
            loop {
                print!("{}  ", a[i][j]);
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }
        println!();
        i += 1;
    }
}
pub fn print_col_wise() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    while i < c {
        if i % 2 == 0 {
            let mut j = 0;
            while j < r {
                print!("{}  ", a[j][i]);
                j += 1;
            }
        } else {
            let mut j = (r - 1) as i32;
            while j >= 0 {
                print!("{}  ", a[j as usize][i]);
                j -= 1;
            }
        }
        println!();
        i += 1;
    }
}
pub fn bounday_element() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    //First row r = 0
    while i < c {
        print!("{}  ", a[0][i]);
        i += 1;
    }
    println!();
    let mut i = 1;
    while i < r {
        print!("{}  ", a[i][c - 1]);
        i += 1;
    }
    println!();
    let mut i = (c - 2) as i32;
    while i >= 0 {
        print!("{}  ", a[r - 1][i as usize]);
        i -= 1;
    }
    println!();
    let mut i = (r - 2) as i32;
    while i >= 1 {
        print!("{}  ", a[i as usize][0]);
        i -= 1;
    }
    println!();
}
pub fn two_d_array() {
    row_to_col();
    col_to_row();
    wave_print_row_wise();
    print_col_wise();
    bounday_element();
    bounday_elements();
    bounday_element_row();
}

pub fn bounday_element_row() {
    let a = [[1, 2, 3, 1], [4, 5, 6, 2], [7, 8, 9, 3]];
    let r = 3;
    let c = 4;
    let mut i = 0;
    //top to bottom   c = 0 ;
    while i < r {
        print!("{}  ", a[i][0]);

        i += 1;
    }
    println!();
    let mut i = 1;
    //right to left  r = 1 ;
    while i < c {
        print!("{}  ", a[r - 1][i]);
        i += 1;
    }
    println!();
    let mut i = 1 as i32;
    while i >= 0 {
        print!("{}  ", a[i as usize][c - 1]);
        i -= 1;
    }
    println!();
    let mut i = (c - 2) as i32;
    while i >= 1 {
        print!("{}  ", a[0][i as usize]);
        i -= 1;
    }
    println!()
}
