/// Sort an array into ascending order using Insertion sort.
pub fn sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    // ループ不変条件： arr[0..j-1]までがソート済み
    for j in 1..arr.len() {
        // key：次にソート済みの部分列に挿入する要素の値
        // i:  0 ~ j の範囲でkeyが挿入される位置のインデックス
        let key = arr[j]; 
        let mut i = j;
        while i > 0 && arr[i-1] > key {
            arr[i] = arr[i-1];
            i -= 1;
        }
        arr[i] = key; // break後の巻き戻し
    }
}
