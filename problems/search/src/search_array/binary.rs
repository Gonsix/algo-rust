use std::fmt::Debug;

pub fn binary_search<T> (a: &[T], key: T) -> Option<usize> 
where 
    T: PartialEq + PartialOrd + Debug + Copy
{

    let mut left = 0_usize;
    let mut right = a.len(); // right は探索範囲の外側のインデックス
    let mut mid: usize;
    while left < right { 
        mid = (left + right)/2;

        if a[mid] == key {
            return Some(mid);
        }

        else if a[mid] < key {
            left = mid + 1;
        }
        else if a[mid] > key {
            right = mid;
        }
    }


    return None;
}
