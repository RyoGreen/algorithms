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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = Solution::min_depth(node.borrow().left.clone());
            let right = Solution::min_depth(node.borrow().right.clone());
            if left == 0 {
                return right + 1;
            }
            if right == 0 {
                return left + 1;
            }
            cmp::min(left, right) + 1
        } else {
            0
        }
    }
}
