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
pub fn pattern_printing() {
    one_to_n_number(5);
    one_to_n_number_star(6);
    one_to_n_number_of_star(5);
    one_to_n_number_of_four_star(5);
    m_star_print_evry_row(1000, 5);
}
