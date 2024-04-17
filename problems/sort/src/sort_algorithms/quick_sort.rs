

pub fn sort<T>(arr: &mut [T]) 
where
    T: PartialOrd +  Copy
{
    quick_sort(arr, 0, arr.len()-1);
}



fn quick_sort<T> (arr: &mut [T], left: usize, right: usize) 
where
    T: PartialOrd +  Copy
{
    if left < right {
        let q = partition(arr, left, right);
        if q > 0 { // qが0より大きい場合のみ、q-1を計算
            quick_sort(arr, left, q.checked_sub(1).unwrap());
        }
        quick_sort(arr, q+1, right);
    }
}

fn partition<T> (arr: &mut [T], left: usize, right: usize) -> usize
where
    T: PartialOrd + Copy
{
    let x = arr[right]; // 0番目の要素を基準とする

    // println!("left: {}", left);
    let mut i: i64 =  (left as i64) - 1;  // 不変条件： arr[left] ~ arr[i]  までが x より小さい値の要素
    for j in (left)..(right) {
        if arr[j] <= x {
            i += 1;
            arr.swap(i as usize, j);
        }
    }
    arr.swap((i+1) as usize, right);


    return (i+1) as usize;
}
