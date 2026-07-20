#![allow(dead_code)]
use std::io;

//Sorting
// To rearrange data in a specfic order
// Ascending / Descending

//Bubble Sort
//If two neighbours / adjacent element are in the wrong order => swap them
//
pub fn selection_sorting() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let mut arr: Vec<i32> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        let mut mn = arr[i];
        let mut loc = i;

        while j < n {
            if arr[j] < mn {
                mn = arr[j];
                loc = j;
            }
            j += 1;
        }
        arr.swap(i, loc);
        print!("Pass {}: ", i + 1);
        for x in &arr {
            print!("{} ", x)
        }
        println!("min_selected : {}", mn);
        i += 1;
    }
}
pub fn bubble_sort() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let mut arr: Vec<i32> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = n - 1;
    let mut pass = 1;

    while i >= 1 {
        let mut j = 0;
        let mut swaps = 0;
        while j < i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swaps += 1;
            }

            j += 1;
        }

        print!("Pass {}: ", pass);
        for x in &arr {
            print!("{} ", x)
        }
        println!(", swaps = {} ", swaps);
        if swaps == 0 {
            break;
        }
        pass += 1;
        i -= 1;
    }
}

pub fn sorting() {
    /*     selection_sorting(); */
    bubble_sort();
}
