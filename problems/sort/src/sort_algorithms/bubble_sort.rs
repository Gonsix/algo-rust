/// Sort an array into ascending order.
pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    let mut n = arr.len();
    if n == 0 {
        return;
    }
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i-1] > arr[i] { // デカい値の要素を後ろへ持っていく
                arr.swap(i, i-1);
                swapped = true;
            }
        }
        n -= 1
    }

}

#[cfg(test)]

mod tests {

    use super::sort;

    #[test]
    fn test_sort_empty_array() {
        let mut empty: Vec<i32> = vec![];
        sort(&mut empty);
        assert_eq!(empty, vec![]);
    }
}
