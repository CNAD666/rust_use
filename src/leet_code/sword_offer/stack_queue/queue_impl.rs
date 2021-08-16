/// 用两个栈实现一个队列。队列的声明如下，请实现它的两个函数 appendTail 和 deleteHead ，
/// 分别完成在队列尾部插入整数和在队列头部删除整数的功能。(若队列中没有元素，deleteHead操作返回 -1 )
///
/// 输入：
/// ["CQueue","appendTail","deleteHead","deleteHead"]
/// [[],[3],[],[]]
/// 输出：[null,null,3,-1]
/// -----------------------解释-----------------------------
/// ["CQueue","appendTail","deleteHead","deleteHead"] 这里是要执行的方法，从左到右执行
/// [[],[3],[],[]]对应上面的方法，是上面方法的参数。CQueue和deleteHead方法不需要指定数字，只有添加才需要指定数字
/// 1.创建队列，返回值为null
/// 2.将3压入栈，返回值为null
/// 3.将栈底的元素删除，也就是消息队列中先进来的元素，所以是deleteHead，返回该元素的数值，所以为3
/// 4.继续删除栈底的元素，但是没有元素了，所以返回-1
/// 所以就有了下面的输出 输出：[null,null,3,-1]
///
/// 输入：
/// ["CQueue","deleteHead","appendTail","appendTail","deleteHead","deleteHead"]
/// [[],[],[5],[2],[],[]]
/// 输出：[null,-1,null,null,5,2]

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */
pub fn main() {
    let mut obj = CQueue::new();
    obj.append_tail(12);
    let return_value = obj.delete_head();
    println!("{} ", return_value);
}

struct CQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        CQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.in_stack.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(value) = self.in_stack.pop() {
                self.out_stack.push(value);
            }
        }
        self.out_stack.pop().unwrap_or(-1)
    }
}

