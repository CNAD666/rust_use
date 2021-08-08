use crate::demo::call_back::{call_back, call_back_set};
use crate::demo::struct_use::struct_use;
use crate::demo::option_use::option_use;

pub fn test() {
    let default = 3;

    println!("打印开始");
    println!("------------------------------------------");
    match default {
        //闭包写法
        1 => {
            call_back();
            call_back_set();
        }
        2 => struct_use(),
        3 => option_use(),
        _ => {}
    }
    println!("------------------------------------------");
    println!("打印结束");
}