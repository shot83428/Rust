use std::cell::RefCell;
use std::rc::Rc;
use crate::models::treenode::TreeNode;
struct Solution1372 {}

static mut result: i32 = 0;
impl Solution1372 {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let va = node.borrow();
            let leftv = &va.left;
            let rightv = &va.right;
            Self::left(leftv.clone(), 1);
            Self::right(rightv.clone(), 1);
        }
        return unsafe { result };
    }
    fn left(root: Option<Rc<RefCell<TreeNode>>>, count: i32) {
        if let Some(node) = root {
            unsafe { result = std::cmp::max(result, count) };

            let va = node.borrow();
            let leftv = &va.left;
            let rightv = &va.right;
            Self::left(leftv.clone(), 1);
            Self::right(rightv.clone(), count + 1);
        }
    }
    fn right(root: Option<Rc<RefCell<TreeNode>>>, count: i32) {
        if let Some(node) = root {
            unsafe { result = std::cmp::max(result, count) };

            let va = node.borrow();
            let leftv = &va.left;
            let rightv = &va.right;
            Self::left(leftv.clone(), count+1);
            Self::right(rightv.clone(), 1);
        }
    }
}


// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }