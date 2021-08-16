use std::option::Option::Some;

/// 定义栈的数据结构，请在该类型中实现一个能够得到栈的最小元素的 min 函数在该栈中，
/// 调用 min、push 及 pop 的时间复杂度都是 O(1)。
///
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.min();   --> 返回 -3.
/// minStack.pop();
/// minStack.top();      --> 返回 0.
/// minStack.min();   --> 返回 -2.

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */
pub fn main() {
    let mut obj = MinStack::new();
    obj.push(-3);
    obj.push(0);
    obj.push(-1);
    obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: i32 = obj.min();

    println!("top：{}", ret_3);
    println!("min：{}", ret_4);
}


struct MinStack {
    stack_real: Vec<i32>,
    stack_min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack_real: Vec::new(),
            stack_min: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack_min.is_empty() {
            self.stack_real.push(x);
            self.stack_min.push(x);
        } else if self.stack_min[self.stack_min.len() - 1] >= x {
            self.stack_real.push(x);
            self.stack_min.push(x);
        } else {
            self.stack_real.push(x);
        }
    }

    fn pop(&mut self) {
        match self.stack_real.pop() {
            Some(value)  if !self.stack_min.is_empty() && value == self.stack_min[self.stack_min.len() - 1] => {
                self.stack_min.pop();
            }
            _ => {}
        }
    }

    fn top(&self) -> i32 {
        if !self.stack_real.is_empty() {
            return self.stack_real[self.stack_real.len() - 1];
        }
        -1
    }

    fn min(&self) -> i32 {
        if !self.stack_min.is_empty() {
            return self.stack_min[self.stack_min.len() - 1];
        }
        -1
    }
}

