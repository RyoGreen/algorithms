#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
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
