use binary_search_tree::BinSearchTree;
use proconio::input;

fn main() {
    let mut bst_tree = BinSearchTree::new();



    loop {
        // print!("Input > ");
        input! {
            command: String,
            key: i32
        }

        match &*command {
            "insert" => {
                // println!("insert: {}", key);
                bst_tree.insert(key);
            }
            "delete" => {   
                
                match bst_tree.delete(key) {
                    Some(bst_node) => {
                        // println!("Deleted {}", bst_node.key);
                        std::mem::drop(bst_node); // 即座に解放
                    }
                    None => {
                        // println!("Not in the Tree.")
                    }
                }

            }
            "find" => {
                match bst_tree.search(key) {
                    Some(_) =>  {println!("Found !");}
                    None => {println!("Not found.")}
                }
                continue;
            }

            "print" => {
                // bst_tree.print();
            }

            "END" => {
                return;
            }
            _ => {
                continue;
            }
        }
        bst_tree.print();
    }

}
