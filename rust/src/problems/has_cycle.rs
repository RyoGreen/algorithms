pub struct Solution;
use super::structs::ListNode;

impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let (mut slow, mut fast) = (head.as_ref(), head.as_ref());
        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            slow = slow_node.next.as_ref();
            fast = fast_node.next.as_ref().and_then(|n| n.next.as_ref());

            if slow == fast && slow.is_some() {
                return true;
            }
        }
        return false;
    }
}
