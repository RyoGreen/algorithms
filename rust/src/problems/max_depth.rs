use super::structs::TreeNode;
use std::{cell::RefCell, cmp, rc::Rc};

pub struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                cmp::max(
                    Self::max_depth(node.borrow().left.clone()),
                    Self::max_depth(node.borrow().right.clone()),
                ) + 1
            }
            None => 0,
        }
    }
}
