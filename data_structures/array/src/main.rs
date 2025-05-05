use array::unsafe_linked_list::LinkedList;
fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(10_i32);

    list.iterate();
}
