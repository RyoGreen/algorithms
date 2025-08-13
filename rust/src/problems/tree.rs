use super::structs::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = Solution::max_depth(node.borrow().left.clone());
            let right = Solution::max_depth(node.borrow().right.clone());
            cmp::max(left, right) + 1
        } else {
            0
        }
    }
}
