use heap::MaxHeap; // 理想のインポートができた　

fn main() {

    let mut  heap: MaxHeap<i32> = MaxHeap::new();

    let vec1 = vec![40, 30, 50, 10, 60, 20];

    for i in 0..vec1.len() {
        heap.push(vec1[i]);
    }

    heap.print();

    match heap.pop() {
        Some(x) => {
            println!("PoP! {} ", x);
        }
        None => {

        }
    }

    heap.print();

}
