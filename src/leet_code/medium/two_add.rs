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


    let mut temp = &result;
    while let Some(node) = temp {
        println!("{}", node.val);
        temp = &node.next
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut t = (l1, l2, 0, 0); // list1, list2, sum, carry

        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(node), None, _, carry) | (None, Some(node), _, carry) => {
                    if node.val + carry >= 10 {
                        (node.next, None, node.val + carry - 10, 1)
                    } else {
                        (node.next, None, node.val + carry, 0)
                    }
                }
                (Some(node1), Some(node2), _, carry) => {
                    if node1.val + node2.val + carry >= 10 {
                        (node1.next, node2.next, node1.val + node2.val + carry - 10, 1)
                    } else {
                        (node1.next, node2.next, node1.val + node2.val + carry, 0)
                    }
                }
            };
            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
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
    let mut list_node = ListNode::new(nums[0]);
    for index in 1..nums.len() {
        list_node.append(nums[index]);
    }

    Some(Box::new(list_node))
}

impl ListNode {
    // 想修改节点，必须返回可变借用
    pub fn last_node(&mut self) -> &mut Self {
        return if let Some(ref mut node) = self.next {
            node.last_node()
        } else {
            self
        };
    }

    // 追加节点
    pub fn append(&mut self, val: i32) {
        let _node = ListNode::new(val);
        self.last_node().next = Some(Box::new(_node));
    }
}



