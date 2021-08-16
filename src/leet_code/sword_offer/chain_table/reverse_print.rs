use std::option::Option::Some;

use crate::leet_code::common::chain_table::ListNode;

/// 从尾到头打印链表
///
/// 输入：head = [1,3,2]
/// 输出：[2,3,1]
pub fn main() {
    let test_node = ListNode::produce_chain(vec![1, 3, 2]);

    let result = Solution::reverse_print(test_node);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut origin = head;
        let mut result = vec![];
        while let Some(value) = origin {
            result.insert(0, value.val);
            origin = value.next;
        }
        result
    }
}

