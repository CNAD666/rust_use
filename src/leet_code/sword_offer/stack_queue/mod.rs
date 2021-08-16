pub mod queue_impl;
pub mod stack_impl;

pub fn main() {
    let default = 1;

    match default {
        //队列实现
        1 => queue_impl::main(),
        //堆栈实现
        2 => stack_impl::main(),
        _ => {}
    }
}
