use std::{os::unix::process::parent_id, ptr::null_mut};




#[derive(Debug)]
pub struct BstNode {
    pub key: i32,
    pub left: *mut Self,
    pub right: *mut Self,
    pub parent: *mut Self
}

impl BstNode {
    // ptr 以下のノードでキーの値が一番小さいのを探す
    fn get_minimum(mut ptr: *mut BstNode) -> *mut BstNode {
        unsafe {
            while !(*ptr).left.is_null() {
                ptr = (*ptr).left;
            }
        }
        return ptr;
    }
}


pub struct BinSearchTree {
    pub root_node: *mut BstNode
}

impl BinSearchTree {
    pub fn new() -> Self {
        return BinSearchTree{ 
            root_node: std::ptr::null_mut()
        }
    }

    /// 左 < 親　<= 右
    pub fn insert(&mut self, new_key: i32) {
        let mut new_node = Box::new(BstNode {
            key: new_key,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
        });

        let z = Box::into_raw(new_node);

        let mut y: *mut BstNode = std::ptr::null_mut();
        let mut x: *mut BstNode = self.root_node;

        while !x.is_null() {
            y = x;
            unsafe {
                if new_key < (*x).key {
                    x = (*x).left;
                }
                else {
                    x = (*x).right;
                }
            }
        }
        unsafe {
            (*z).parent = y;
        }

        // if the Tree is empty.
        unsafe {
            if y.is_null() {
                self.root_node = z;
            }
            else if new_key < (*y).key {
                (*y).left = z;
            }
            else {
                (*y).right = z;
            }
        }
        
    }


    /// In-order walk tree.
    /// ソート済の配列を作成するのにも利用できる。 
    pub fn inorder(&self) -> Option<Vec<i32>>  {

        if self.root_node.is_null() {
            return None;
        }
        let mut stack: Vec<*mut BstNode> = Vec::new();
        let mut result: Vec<i32> = Vec::new();
        let mut current: *mut BstNode = self.root_node;
    
        while !current.is_null() || !stack.is_empty() {
            // 現在のノードがnullでない間、スタックに追加し続け、左に進みます。
            while !current.is_null() {
                stack.push(current);
                unsafe {
                    current = (*current).left;
                }
            }
            // 左の子がなくなったら、スタックからポップし、値を記録し、右へ進みます。
            current = stack.pop().unwrap();
            unsafe {
                result.push((*current).key);
                current = (*current).right;
            }
        }
    
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn print(&self) {
        match self.inorder() {
            Some(result) => {
                result.iter().for_each(|e| print!("{} ", e));
                println!();
            }
            None => {
                println!("Empty Tree !");
            }
        }
    }


    /// Search in the Binary search tree.
    pub fn search(&self, key: i32) -> Option<*mut BstNode> {
        let mut current = self.root_node;

        while !current.is_null() {
            unsafe {
                if key == (*current).key {
                    return Some(current);
                }

                else if key < (*current).key {
                    current = (*current).left;
                }
                else {
                    current = (*current).right;
                }
                
            }
        }
        
        return None;
    }



    /// uの親との関係を、vとparentの関係に置き換える
    fn transparent(&mut self, u: *mut BstNode, v: *mut BstNode) {
        unsafe {
            if (*u).parent.is_null() { // もしくは self.root_node == u
                self.root_node = v;
            }
            else if (*(*u).parent).left == u { // 左の子なら
                (*(*u).parent).left = v;
            }
            else { // 右の子なら
                (*(*u).parent).right = v;
            }

            if !v.is_null() {
                (*v).parent = (*u).parent;
            }
        }
    }

    /// tree から　keyの値をもつ要素をdelete ！
    /// 二分探索木の中で同じ値を持つ要素が複数存在しないことを想定している
    /// 参照： MIT アルゴリズムイントロダクションの二分探索セクション
    pub fn delete(&mut self, key: i32) -> Option<Box<BstNode>> {
        if let Some(z) = self.search(key) {
            unsafe {
                if (*z).left.is_null() {
                    self.transparent(z, (*z).right);
                }
                else if (*z).right.is_null() {
                    self.transparent(z, (*z).left);
                }
                else { // 消去対象のノードが左右どちらにも子を持つ場合
                    let y = BstNode::get_minimum((*z).right);
                    if (*y).parent != z {
                        self.transparent(y, (*y).right);
                        (*y).right = (*z).right;
                       (*(*y).right).parent = y;
                    }
                    self.transparent(z, y);
                    (*y).left = (*z).left;
                    (*(*y).left).parent = y;
                }
                return Some(Box::from_raw(z));
            }
        }
        else {
            None
        }

    }


}



