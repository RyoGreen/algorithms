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
}
