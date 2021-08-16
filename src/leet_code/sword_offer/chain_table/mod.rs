mod reverse_print;
mod reverse_list;
mod chain_copy;


pub fn main() {
    let default = 1;

    match default {
        // 链表反转打印
        1 => reverse_print::main(),
        // 链表反转
        2 => reverse_list::main(),
        // 复杂链表复制,暂不支持rust
        3 => chain_copy::main(),
        _ => {}
    }
}


