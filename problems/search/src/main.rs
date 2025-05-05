use search::binary;
use search::linear::linear_search;
fn main() {

    let vec1 = vec![1,2,3,4];
    let key = -1;

    match linear_search(&vec1, key) {
        Some(index) => {
            println!("Found: {}", index);
        }
        None => {
            println!("Not Found");
        }
    }
    // match binary::binary_search(&vec1, key) {
    //     Some(index) => {
    //         println!("Found: {}", index);
    //     }
    //     None => {
    //         println!("Not Found");
    //     }
    // }
}
