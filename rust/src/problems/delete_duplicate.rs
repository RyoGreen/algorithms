pub struct Solution;
use super::structs::ListNode;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut()?;

        while let Some(next) = current.next.as_mut() {
            if current.val == next.val {
                current.next = next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        head
    }

    // Input: head = [1,2,3,3,3,4,4,5]
    // Output: [1,2,5]
    pub fn delete_duplicates_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Box::new(ListNode::new(0));
        let mut current = &mut new_head;
        let mut head = head;

        while let Some(mut node) = head {
            let mut is_duplicate = false;

            while let Some(next_node) = &node.next {
                if node.val == next_node.val {
                    is_duplicate = true;
                    node.next = node.next.take().unwrap().next;
                } else {
                    break;
                }
            }

            if !is_duplicate {
                current.next = Some(Box::new(ListNode::new(node.val)));
                current = current.next.as_mut().unwrap();
            }

            head = node.next.take();
        }
        new_head.next
    }
}
