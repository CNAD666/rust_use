// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl ListNode {
    /// 想修改节点，必须返回可变借用
    pub fn last_node(&mut self) -> &mut Self {
        return if let Some(ref mut node) = self.next {
            node.last_node()
        } else {
            self
        };
    }

    /// 追加节点
    pub fn append(&mut self, val: i32) {
        let _node = ListNode::new(val);
        self.last_node().next = Some(Box::new(_node));
    }

    /// 数组转链表
    pub fn produce_chain(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list_node = ListNode::new(nums[0]);
        for index in 1..nums.len() {
            list_node.append(nums[index]);
        }

        Some(Box::new(list_node))
    }
}