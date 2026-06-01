pub fn one_to_n_number(n: u32) {
    for i in 1..=n {
        println!("{i:?}");
    }
}

pub fn one_to_n_number_star(n: u32) {
    for i in 1..=n {
        println!("{}", "*".repeat(i as usize));
    }
}
pub fn pattern_printing() {
    one_to_n_number(5);
    one_to_n_number_star(6);
}
