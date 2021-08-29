mod chain_table;
mod stack_queue;
mod string;

pub fn main() {
    let default = 3;

    match default {
        //堆栈和队列篇
        1 => stack_queue::main(),
        //链表篇
        2 => chain_table::main(),
        //字符串篇
        3 => string::main(),
        _ => {}
    }
}

