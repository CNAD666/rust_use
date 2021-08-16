use crate::leet_code::common::chain_table::ListNode;

pub fn main() {
    let list = ListNode::produce_chain(vec![1, 3, 2]);
    let reverse_list = Solution::reverse_list(list);
    println!("{:?}", reverse_list);
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut origin = head;
        let mut reverse = None;
        while let Some(mut value) = origin {
            origin = value.next.take();
            value.next = reverse;
            reverse = Some(value);
        }
        reverse
    }
}