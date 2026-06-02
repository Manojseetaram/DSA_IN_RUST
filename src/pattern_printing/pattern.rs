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
pub fn pattern_printing() {
    one_to_n_number(5);
    one_to_n_number_star(6);
    one_to_n_number_of_star(5);
}
