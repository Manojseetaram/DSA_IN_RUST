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
pub fn two_d_array() {
    row_to_col();
    col_to_row();
    wave_print_row_wise();
    print_col_wise();
}
