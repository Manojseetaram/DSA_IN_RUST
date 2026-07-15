mod arrays;
mod loops;
mod pattern_printing;
mod two_d_array;
fn main() {
    #![allow(dead_code)]
    loops::loop_fn::loops_function();
    pattern_printing::pattern::pattern_printing();
    arrays::arr::arrays();
    two_d_array::two_d::two_d_array();
}
