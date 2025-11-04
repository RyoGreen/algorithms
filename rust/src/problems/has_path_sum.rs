use super::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Solution::dfs(root, target_sum, 0)
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32) -> bool {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();
            let new_sum = current_sum + node_ref.val;

            if node_ref.left.is_none() && node_ref.right.is_none() {
                return new_sum == target_sum;
            }
            return Solution::dfs(node_ref.right.clone(), target_sum, new_sum)
                || Solution::dfs(node_ref.left.clone(), target_sum, new_sum);
        }
        return false;
    }
}
