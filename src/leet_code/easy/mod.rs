mod two_sum;
mod queue_impl;
mod stack_impl;

pub fn main() {
    let default =3;

    match default {
        //俩数相加
        1 => two_sum::main(),
        //两个栈实现一个队列
        2 => queue_impl::main(),
        //实现最小栈
        3 => stack_impl::main(),
        _ => {}
    }
}

