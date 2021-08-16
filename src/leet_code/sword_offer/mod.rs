mod chain_table;
mod stack_queue;

pub fn main() {
    let default = 2;

    match default {
        //堆栈和队列篇
        1 => stack_queue::main(),
        //链表篇
        2 => chain_table::main(),
        _ => {}
    }
}

