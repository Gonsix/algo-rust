use sort::insertion_sort;
use sort::merge_sort;
use sort::bubble_sort;
use sort::selection_sort;
use sort::quick_sort;
fn main() {
    insertion_sort::sort();
    selection_sort::sort();
    bubble_sort::sort();
    merge_sort::sort();
    quick_sort::sort();
}
