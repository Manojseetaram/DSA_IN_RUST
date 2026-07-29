mod arrays;
mod loops;
mod pattern_printing;
mod prefixsum;
mod search;
mod sets_and_maps;
mod sorting;
mod strings;
mod subarrays;
mod timeandspace;
mod two_d_array;
mod two_pointers;
fn main() {
    #![allow(dead_code)]
    loops::loop_fn::loops_function();
    pattern_printing::pattern::pattern_printing();
    arrays::arr::arrays();
    two_d_array::two_d::two_d_array();
    strings::str::strings();
    search::search::serching();
    sorting::sort::sorting();
    timeandspace::time::timeandspace();
    prefixsum::prefix::prefix_sum();
    subarrays::sub::subarray();
    sets_and_maps::sets::sets_and_maps();
    two_pointers::two::two_pointers();
}
