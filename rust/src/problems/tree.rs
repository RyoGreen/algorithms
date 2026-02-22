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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        // max_depth
        let node4 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node4),
            right: None,
        }));
        let node3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node2),
            right: Some(node3),
        })));
        assert_eq!(Solution::max_depth(root), 3);

        // min_depth
        let node4 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node4),
            right: None,
        }));
        let node3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node2),
            right: Some(node3),
        })));
        assert_eq!(Solution::min_depth(root), 2);

        // merge_trees
        let node4 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node4),
            right: None,
        }));
        let node3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node2),
            right: Some(node3),
        })));
        let node5 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        }));
        let node6 = Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        }));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node5),
            right: Some(node6),
        })));
        let expected_root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        })));

        let merged = Solution::merge_trees(root1, root2);
        assert_eq!(merged, expected_root);
    }
}
