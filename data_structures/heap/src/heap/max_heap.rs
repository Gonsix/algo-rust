use std::{clone, fmt::Debug};

use num_traits::Bounded;

fn parent(i: usize) -> Option<usize>  {
    if i > 0 {
        return Some(i/2);
    }
    else {
        return None;
    }
}

fn left(i: usize) -> usize {
    return 2*i;
}

fn right(i: usize) -> usize {
    return 2*i + 1;
}

pub struct MaxHeap <T> 
where 
    T: PartialOrd + Bounded + Copy
{
    data: Vec<T>,
}


impl<T> MaxHeap<T> 
where
    T: PartialOrd  + Bounded + Debug + Copy
{

    pub fn downheap(&mut self, i: usize) {
        let heap_size = self.size(); // ０番目の要素はカウントしない
        self.downheap_in_size(i, heap_size);
    }

    // 対象区間を選択できる Down heap
    fn downheap_in_size(&mut self, i: usize, size: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest: usize = i;

        if l <= size && self.data[l] > self.data[i] {
            largest = l;
        }
        if r <= size && self.data[r] > self.data[largest] {
            largest = r;            
        }

        if largest != i  {
            self.data.swap(i, largest);
            self.downheap_in_size(largest, size);
        }
    }

    pub fn upheap(&mut self, mut i:usize) {

        // while 0 < i {}
        // if  i ≤ 0: break
        while let Some(p) = parent(i) {


            if self.data[p] < self.data[i] {
                self.data.swap(i, p);
            }
            else {
                break;
            }
            i = p;
        }

    }

    pub fn new()  -> Self {
        let mut heap =  MaxHeap {
            data : Vec::new()
        };
        // 0番目の要素を番兵で初期化
        heap.push(T::max_value());
        return heap;
    }

    pub fn push(&mut self, value: T) {
        let old_heap_size = self.data.len();
        self.data.push(value);
        self.upheap(old_heap_size);
    }


    pub fn pop(&mut self) -> Option<T> {
        let heap_size = self.data.len() - 1;
        if self.data.len() == 1 {
            return None;
        }
        else {
            let val = self.data[1];
            self.data[1] = self.data[heap_size];
            self.data.pop();
            self.downheap(1); // ルートをダウン・ヒープ
            
            return Some(val);

        }

    }
    
    pub fn clear(&mut self) {
        // TODO: Implement this method

    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.len() > 1 {
            return Some(&self.data[1]);
        }
        else {
            return None;
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.data.len() > 1 {
            return Some(&mut self.data[1]);
        }
        else {
            return None;
        }    
    }

    /// Consumes the BinaryHeap and returns a vector in sorted (ascending) order.
    /// ヒープソートを利用
    pub fn into_sorted_vec(&mut self) -> Vec<T> {

        self.heap_sort();

        // インデックス０の要素を除いたベクターを新たに作成
        let sorted_vec = self.data[1..self.data.len()].to_vec();
        return sorted_vec;
        
    }

    /// ヒープソートを行う
    pub fn heap_sort(&mut self) {
        let mut i: usize = self.size();
        while i >= 2 {
            // ルートと末端の要素を入れ替える
            self.data.swap(1, i);
            i -= 1;
            self.downheap_in_size(1, i);
        }
    }

    //    ####  Utilities  ####
    
    
    pub fn print(&self) {
        for i in 1..self.data.len() {
            println!("[{}]-> {:?}", i, self.data[i]);
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.data.len() {
            1 => true,
            _ => false
        }
    }

    pub fn size(&self) -> usize {
        return self.data.len() -1;
    }


}


impl<T> From<Vec<T>> for MaxHeap<T>
where
    T: PartialOrd + Copy + Bounded
{
    /// Build Max heap from a vector
    fn from(vector: Vec<T>) -> Self {
        MaxHeap {
            data: vector
        }
    }

    
}
