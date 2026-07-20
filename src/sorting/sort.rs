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
        i += 1;
    }
    println!("{:?}", arr);
}
pub fn sorting() {
    selection_sorting();
}
