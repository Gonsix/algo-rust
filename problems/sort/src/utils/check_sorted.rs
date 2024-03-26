/// Check whether the array is sorted or not.
pub fn check_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    // ループのではなく、イテレーターを使用する
    arr.windows(2).all(|w| w[0] <= w[1])
}


