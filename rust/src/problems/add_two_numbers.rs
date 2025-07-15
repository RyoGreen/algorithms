use crate::problems::structs::ListNode;

pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();

        while p1.is_some() || p2.is_some() || carry > 0 {
            let x = p1.map_or(0, |node| node.val);
            let y = p2.map_or(0, |node| node.val);

            let sum = x + y + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            if let Some(node) = p1 {
                p1 = node.next.as_ref();
            }
            if let Some(node) = p2 {
                p2 = node.next.as_ref();
            }
        }

        dummy_head.next
    }
}
