use rust::problems::structs::TreeNode;
use rust::problems::tree::Solution;
use std::cell::RefCell;
use std::rc::Rc;

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
}
