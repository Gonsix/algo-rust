use sort::insertion_sort;
use sort::merge_sort;
use sort::bubble_sort;
use sort::selection_sort;
use sort::quick_sort;
fn main() {

    let mut vec1 = vec![5, 3, 1, 90, 0];
    println!("{:?}", vec1);

    bubble_sort::sort(&mut vec1);
    println!("{:?}", vec1);


    insertion_sort::sort();
    selection_sort::sort();
    // bubble_sort::sort();
    merge_sort::sort();
    quick_sort::sort();

}
