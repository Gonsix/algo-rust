use sort::bubble_sort;
use sort::insertion_sort;
use sort::merge_sort;
// use sort::bubble_sort;
use sort::selection_sort;
use sort::quick_sort;
use sort::utils::check_sorted;
use sort::utils::{gen_random_usize, gen_random_vec_i32};

#[allow(unused_imports)]
fn main() {
    let mut vec1 = vec![5, 3, 1, 90, 0];

    // println!("{:?}", vec1);

    // insertion_sort::sort(&mut vec1);
    // // println!("{:?}", vec1);
    // println!("{}", check_sorted(&vec1));


    // // insertion_sort::sort();
    // selection_sort::sort();
    // // bubble_sort::sort();
    // merge_sort::sort();
    // quick_sort::sort();

    let n = gen_random_usize(10_usize.pow(4));
    println!("Generated: {}", n);

    let mut rand_vec = gen_random_vec_i32(n, 10_i32.pow(6));

    println!("sample element: {:?}", rand_vec[5]);

    bubble_sort::sort(& mut rand_vec);
    // println!("{:?}", rand_vec);

    println!("{}", check_sorted(&rand_vec));

    // これでソートのテストができる！
}
