use heap::MaxHeap; // 理想のインポートができた　

fn main() {

    let mut  heap: MaxHeap<i32> = MaxHeap::new();

    let vec1 = vec![40, 30, 50, 10, 60, 20];

    for i in 0..vec1.len() {
        heap.push(vec1[i]);
    }

    println!("Before Sort:");
    heap.print();

    println!("Size: {}", heap.size());

    let sorted_vec = heap.into_sorted_vec();


    println!("After Sorted: {:#?}", sorted_vec);

    // match heap.pop() {
    //     Some(x) => {
    //         println!("PoP! {} ", x);
    //     }
    //     None => {

    //     }
    // }

    // heap.print();

    // println!("Size: {}", heap.size());
    println!("Empty: {}", heap.is_empty());

}
