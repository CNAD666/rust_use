use std::option::Option::Some;

/// 给你两个非空 的链表，表示两个非负的整数。它们每位数字都是按照逆序的方式存储的，并且每个节点只能存储一位数字。
/// 请你将两个数相加，并以相同形式返回一个表示和的链表。
/// 你可以假设除了数字 0 之外，这两个数都不会以 0开头。
///
/// 输入：l1 = [2,4,3], l2 = [5,6,4]
/// 输出：[7,0,8]
/// 解释：342 + 465 = 807.
pub fn main() {
    let one = produce_node(vec![2, 4, 5, 9, 1]);
    let two = produce_node(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(one, two);


    let mut node = &result;
    while node != &None {
        println!("{}", node.as_ref().unwrap().val);

        node = &node.as_ref().unwrap().next;
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(1);
        node.next = Some(Box::new(ListNode::new(1)));
        node.next;
        l1
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn produce_node(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node = Some(Box::new(ListNode::new(nums[0])));
    for index in 1..nums.len() {
        let last_node = ListNode::get_last_mut(&node.unwrap());
        last_node.unwrap().next = Some(Box::new(ListNode::new(nums[index])));
    }

    node
}

impl ListNode {
    // 想修改节点，必须返回可变借用
    pub fn get_last_mut(&mut self) -> &mut Self {
        return if let Some(ref mut boxNode) = self.next {
            boxNode.get_last_mut()
        } else {
            self
        };
    }

    // 追加节点
    pub fn append(&mut self, val: i32) {
        let _node = ListNode::new(val);
        self.get_last_mut().next = Some(Box::new(_node));
    }
}



