mod chain_table;
mod stack_queue;
mod string;
mod seek;

pub fn main() {
    let default = 4;

    match default {
        //堆栈和队列篇
        1 => stack_queue::main(),
        //链表篇
        2 => chain_table::main(),
        //字符串篇
        3 => string::main(),
        //查找算法
        4 => seek::main(),
        _ => {}
    }
}

