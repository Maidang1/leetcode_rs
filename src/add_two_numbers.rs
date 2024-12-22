/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();

    let mut adder = 0;

    while !l1.is_none() || !l2.is_none() {
        if let Option::Some(node) = l1 {
            adder += node.val;
            l1 = node.next;
        }

        if let Option::Some(node) = l2 {
            adder += node.val;
            l2 = node.next;
        }

        if let Option::Some(node) = current {
            node.next = Some(Box::new(ListNode::new(adder % 10)));
            current = node.next.as_mut();
            adder /= 10;
        }
    }

    if adder > 0 {
        if let Option::Some(node) = current {
            node.next = Some(Box::new(ListNode::new(1)));
        }
    }

    head.unwrap().next
}
// @lc code=end
