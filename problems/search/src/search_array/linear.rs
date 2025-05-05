use std::fmt::Debug;

pub fn linear_search<T> (a: &[T], key: T) -> Option<usize> 
where 
    T: PartialEq + PartialOrd + Debug + Copy
{

    for i in 0..a.len() {
        if a[i] == key {
            return Some(i);
        }
    }
    return None;
}
