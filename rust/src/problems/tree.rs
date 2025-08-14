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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }

        if let (Some(r1), Some(r2)) = (root1, root2) {
            let val = r1.borrow().val + r2.borrow().val;

            let left = Solution::merge_trees(r1.borrow().left.clone(), r2.borrow().left.clone());
            let right = Solution::merge_trees(r1.borrow().right.clone(), r2.borrow().right.clone());

            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        } else {
            None
        }
    }
}
