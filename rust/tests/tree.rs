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
