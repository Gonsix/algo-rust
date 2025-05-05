use std::{fmt::Debug, ptr};
struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct LinkedList<T> 
where 
    T: Debug
{
    head: *mut Node<T>,
}

impl<T> LinkedList<T>
where
    T: Debug
{
    pub fn new() -> Self {
        LinkedList { head: ptr::null_mut() } // 最初は番兵を追加
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: self.head,
        }));
        self.head = new_node;
    }

    pub fn iterate(&self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                println!("{:?}", (*current).data);
                current = (*current).next;
            }
        }
    }
}
///
/// LinkedList型の値がスコープを抜ける際に実行するコード
/* chatGPT 返答

- let _ = Box::from_raw(self.head);の行で行っていることは何ですか？
-->  let _ = Box::from_raw(self.head);この行が行っていることは、self.headで指されている生ポインタをBoxに変換し、そのBoxの所有権を取得して即座にスコープを抜けさせることです。この操作によって、Boxがスコープを抜ける際に自動的に行われるメモリの解放がトリガーされます。

ただし、Box::from_rawを使用する際にはいくつかの注意点があります：
生ポインタが正確に1回Box::from_rawに渡されるようにすること。複数回渡すと、同じメモリ領域を解放しようとすることになり、未定義の振る舞いを引き起こす可能性があります。
生ポインタが有効で、適切な型のデータを指していることを確認すること。不正なポインタを渡すと、安全でない振る舞いを引き起こす可能性があります。
生ポインタが、Boxによって確保されたヒープメモリを指していることを保証すること。他のメモリ確保手段を使用して確保されたメモリをBoxで解放しようとすると、不整合が発生する可能性があります。
*/
impl<T> Drop for LinkedList<T> 
where 
    T: Debug
{
    fn drop(&mut self) {
        while !self.head.is_null() {
            unsafe {
                let next = (*self.head).next;
                // println!("{:?}", (*self.head).data);
                let _ = Box::from_raw(self.head);
                self.head = next;
            }
        }
    }
}
