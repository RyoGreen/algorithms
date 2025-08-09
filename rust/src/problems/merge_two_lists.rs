pub struct Solution;
use super::structs::ListNode;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();

        while l1.is_some() || l2.is_some() || carry > 0 {
            let x = l1.map_or(0, |node| node.val);
            let y = l2.map_or(0, |node| node.val);

            if l1.is_some() && l2.is_some() {}
            let sum = x + y + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            if let Some(node) = l1 {
                l1 = node.next.as_ref();
            } else {
                l1 = None;
            }

            if let Some(node) = l2 {
                l2 = node.next.as_ref();
            } else {
                l2 = None;
            }
        }

        dummy_head.next
    }
}
