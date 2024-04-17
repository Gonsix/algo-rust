use sort::bubble_sort;
use sort::gen_random_vec_f64;
use sort::insertion_sort;
use sort::merge_sort;
// use sort::bubble_sort;
use sort::selection_sort;
use sort::quick_sort;
use sort::utils::check_sorted;
use sort::utils::{gen_random_usize, gen_random_vec_i32};

#[allow(unused_imports)]
#[allow(unused_variables)]
fn main() {
    // println!("{:?}", vec1);

    // insertion_sort::sort(&mut vec1);
    // // println!("{:?}", vec1);
    // println!("{}", check_sorted(&vec1));


    // // insertion_sort::sort();
    // selection_sort::sort();
    // // bubble_sort::sort();
    // merge_sort::sort();
    // quick_sort::sort();

    let n = gen_random_usize(10_usize.pow(5));
    let mut rand_vec = gen_random_vec_i32(n, 10_i32.pow(6));
    quick_sort::sort(& mut rand_vec);
    println!("Int array: {}", check_sorted(&rand_vec));

    // Sort a float vector
    let mut rand_vec_float = gen_random_vec_f64(n, 300.0);
    quick_sort::sort(&mut rand_vec_float);
    println!("Float array: {}", check_sorted(&rand_vec_float));

}
