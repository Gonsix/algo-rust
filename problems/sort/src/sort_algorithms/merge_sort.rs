use num_traits::Bounded;

/// Sort an array into ascending order using Merge sort.
pub fn sort<T>(a: &mut [T]) 
where
    T: PartialOrd + Copy + Bounded
{
    merge_sort(a, 0, a.len());
}

/*  Documentation

- 型引数T型の最大値を得るために num-traitsのBounded::max_value()を使用

*/



/// Merge Sort
/// It uses Out of the place algorithm
fn merge_sort<T>(a: &mut [T], left: usize, right: usize) 
where
    T: PartialOrd + Copy + Bounded
{
    if left+1 < right {
        let mid = (left+right)/2;
        merge_sort(a, left, mid);
        merge_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}

fn merge<T>(a: &mut [T], left: usize, mid: usize, right: usize)
where
    T: PartialOrd + Copy + Bounded 
{
    let n1: usize = mid - left;
    let n2: usize = right - mid;

    // 番兵のためにサイズを+1で確保, T型の何らかの値を入れる
    let mut l: Vec<T> = Vec::<T>::new();
    l.resize(n1+1, T::min_value());
    let mut r: Vec<T> = Vec::<T>::new();
    r.resize(n2+1, T::min_value());

    for i in 0..n1 {
        l[i] = a[left+i];
    }
    for i in 0..n2 {
        r[i] = a[mid+i];
    }

     // 番兵
    l[n1] = T::max_value();
    r[n2] = T::max_value();


    let mut i = 0;
    let mut j = 0;

    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        }
        else {
            a[k] = r[j];
            j += 1;
        }
    }




}
