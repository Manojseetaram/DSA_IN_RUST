#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    io,
};
pub fn duplicate_useing_array() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let mut arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;

    while i < n {
        if arr[i] == -1 {
            continue;
        }
        let mut j = i + 1;
        let mut count = 1;
        while j < n {
            if arr[i] == arr[j] {
                count += 1;
                arr[j] = -1;
            }

            j += 1;
        }
        println!("{} : {} times ", arr[i], count);

        i += 1;
    }
}
pub fn duplicate_useing_sets() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();

    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let arr: Vec<i64> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut set: HashSet<i64> = HashSet::new();

    let mut i = 0;
    while i < n {
        if set.contains(&arr[i]) {
            println!("Duplicate found : {}", arr[i]);
            return;
        }
        set.insert(arr[i]);
        i += 1;
    }
    println!("No duplicates");
}
pub fn duplicate_useing_maps() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();
    let n: usize = a.trim().parse().unwrap();

    a.clear();
    io::stdin().read_line(&mut a).unwrap();

    let arr: Vec<i64> = a.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut map: HashMap<i64, i64> = HashMap::new();

    // let mut i = 0;
    // while i < n {
    //     if map.contains_key(&arr[i]) {
    //         *map.get_mut(&arr[i]).unwrap() += 1;
    //     } else {
    //         map.insert(arr[i], 1);
    //     }
    //
    //     i += 1;
    // }
    for &x in &arr {
        *map.entry(x).or_insert(0) += 1;
    }
    println!("{:?}", map);
}

pub fn sets_and_maps() {
    /*     duplicate_useing_sets(); */
    duplicate_useing_maps();
}
