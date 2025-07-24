pub struct Solution;

use super::structs::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;

        dummy.next
    }
}
